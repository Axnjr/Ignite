use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::sync::LazyLock;
use chrono::Local;

const MAX_LOG_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

static LOG_FILE: LazyLock<Mutex<File>> = LazyLock::new(|| {
    Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open("server.log")
            .expect("Unable to open log file")
    )
});

pub fn log_messages_to_log_file(message: &str, level: &str) {

    let timestamp = Local::now().format("%b %d %H:%M:%S").to_string();

    let mut logfile = LOG_FILE
        .lock()
        .unwrap_or_else(|poisoned| {
            let mut logfile = poisoned.into_inner();

            let _ = writeln!(
                logfile, 
                "[{}] | [{}]    :   {}", 
                timestamp, 
                level, 
                "Unable to acquire write access on log file, Recovered from poisoned mutex. Logging might be inconsistent."
            );

            logfile
        })
    ;

    // Truncate the file if its size exceed 10MB
    if let Ok(metadata) = logfile.metadata() {
        if metadata.len() > MAX_LOG_FILE_SIZE {
            println!("Log file exceeded 10 MB. Clearing contents.");
            *logfile = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("server.log")
                .expect("Unable to clear log file!");
        }
    }

    if let Err(e) = writeln!(
        logfile,
        "[{}] | [{}]    :   {}",
        timestamp,
        level,
        message
    ) {
        eprintln!("Failed to write to log file: {}", e);
    }

    println!("{}", message);
}


pub fn log_and_panic<T>(message: &str) -> T {

    // might cause infinite recusion so commented out ..
    // log_messages_to_log_file(message);

    let timestamp = Local::now().format("%b %d %H:%M:%S").to_string();

    let mut logfile = LOG_FILE
        .lock()
        .unwrap_or_else(|poisoned| {

            let mut logfile = poisoned.into_inner();

            let _ = writeln!(
                logfile, 
                "[{}] | [{}]    :   {}", 
                timestamp, 
                "ERROR", 
                "Recovered from poisoned mutex while panicing. Logging might be inconsistent."
            );

            logfile
        })
    ;

    let _ = writeln!(
        logfile,
        "[{}] | [{}]  :   {}", 
        timestamp, 
        "ERROR", 
        message
    );
    
    panic!("{}", message);
}