use std::env;
use dotenv::dotenv;

pub fn get_request_limit_from_plan_name(val: &str) -> usize {
    match val {
        "Hobby" => 500,
        "Pro" => 1000000,
        "StartUp" => 5000000,
        _ => 0,
    }
}

pub fn get_connection_limit_from_plan_name(val: &str) -> usize {
    match val {
        "Hobby" => 10,
        "Pro" => 500,
        "StartUp" => 5000,
        _ => 0,
    }
}

pub fn devlog(val: &str) {
    dotenv().ok();
    let mode = env::var("MODE")
        .unwrap_or(String::from("production"))
    ;
    if mode != "production" {
        println!("{}", val);
    }
}