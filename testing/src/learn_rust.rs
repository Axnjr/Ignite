enum RedHatUserTypes {
    Personal,
    StartUp,
    Enterprize,
    EnterprizePlus
}

#[derive(Clone, Copy)]
enum DeviceGenerations {
    FirstGen,
    SecondGen,
    ThirdGen,
    FourthGen,
    FifthGen,
    SixthGen,
    SeventhGen
}

struct DeviceSpecs {
    memory: f64,
    cpu: i32,
    ram: i32,
    gen: DeviceGenerations,
}

struct RedHatUser {
    name: String,
    user_type: RedHatUserTypes,
    liscense_id: String,
    registered_on: i32, // and date between 1 - 31 for simplicity
    sessions: i32,
    specs: DeviceSpecs,
    user_id: String,
}

trait AllUsers {
    fn get_id(&self) -> String;
    fn get_user_device_specs(&self) -> &DeviceSpecs;
    fn get_sessions(&self) -> i32;
}

impl RedHatUser {
    fn new(
        name: String,
        user_type: RedHatUserTypes,
        license_id: String,
        registered_on_day: i32,
        sessions: i32,
        specs: DeviceSpecs,
        user_id: String,
    ) -> Self {
        Self {
            name,
            user_type,
            liscense_id: license_id, //typo
            registered_on: registered_on_day,
            sessions,
            specs,
            user_id,
        }
    }
}

impl AllUsers for RedHatUser {
    fn get_id(&self) -> String {
        self.user_id.clone()
    }

    fn get_user_device_specs(&self) -> &DeviceSpecs {
        &self.specs
        // DeviceSpecs {
        //     memory: self.specs.memory,
        //     cpu: self.specs.cpu,
        //     ram: self.specs.ram,
        //     gen: self.specs.gen.clone(),
        // }
    }

    fn get_sessions(&self) -> i32 {
        self.sessions
    }
}

fn main() {

    let spec = DeviceSpecs {
        memory: 1.5,
        cpu: 8,
        ram: 16,
        gen: DeviceGenerations::SeventhGen
    };

    let instance: RedHatUser = RedHatUser::new(
        "Radha".to_string(),
        RedHatUserTypes::EnterprizePlus,
        "q2w3e4r5t6y7u8i9".to_string(),
        23,
        23,
        spec,
        "q2w34d5fgyhujiko".to_string(),
    );

    let _s = instance.liscense_id.to_string();

    let _temp = instance.get_id();

    //////////////////////////////////////////////////////////////////////////////

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {

        // ^ TWO METHODS -:

        // ^ 1) BY DE-REFRENCING THE `name` EARLIER ONLY !!
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }

        // ^ 2) BY REFRENCING `&"Ferris"` LATER IN THE `match` !!
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);

    ////////// ^ USE `into_iter()` or directly iterate both transfer ownership to the loop /////////////

    // for name in names.into_iter() {
    //     match name {
    //         "Ferris" => println!("There is a rustacean among us!"),
    //         _ => println!("Hello {}", name),
    //     }
    // }

    // ^ FOR LOOP GETS THE OWNERSHIP OF THE VECTOR NAMES !!
    for name in names {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // ^ GIVES ERROR BECAUSE WE MOVED THE NAMES VECTOR TO THE FOR LOOP, SO ITS NO LONGER AVAILABLE !!
    // ERROR: names has been moved
    // println!("names: {:?}", names);

}