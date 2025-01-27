// - Associated functions: are functions that are defined on a type generally 
// - (which dont take 'self' parameter implicitly)
// - Associated functions don't need to be called with an instance.
// - These functions are generally used like constructors.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // ^ This is an "associated function" because this function is associated with a particular type 'Point'.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // Another associated function, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    // ^ Below is a 'Method' as it takes 'self' as a prameter, 
    // ^ however it is implicitly passed, we dont need to pass it while calling with an instance
    // YES, 'Methods' need to called with their instances using 'dot' operator.
    fn alter_by_two(&self) -> Self {
        Point {
            x: self.x + 2.0,
            y: self.y + 2.0,
        }
    }
}

fn test_point_struct() {
    // ^ Associated functions are called directly using 2 semicolns '::' like below !!
    let temp = Point::origin();
    // Methods need to called on the instance !!
    let _two_point_ahead = temp.alter_by_two();
    // temp.alter_by_two() == temp::alter_by_two(&temp)
}

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

// A Tuple Struct
// ^ 'Tuple Structs' are intialized using `()` not `{}` - REMEMBER !!
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn log_matrix(mat: Matrix) {
    println!("THIS IS THE MATRIX: {:?}", mat);
}

fn matrix_transpose(mat: Matrix) -> Matrix {
    // ^ When creating an instance of a tuple struct, you need to provide the values in parentheses
    Matrix(mat.0, mat.2, mat.1, mat.3) 
}

// struct Cacher<T> where 

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

    //////////////////////////////////////////////////////////////////////////////

    // ^ 'Closures' CANNOT BE GENERIC !!
    // reverse closure with hardcoded types
    let reverse = |pair: (i32, bool)| -> (bool, i32) {
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
    };

    fn reverse_genric<T, U>(pair: (T, U)) -> (U, T) {
        let (generic_param1, generic_param2) = pair;
        (generic_param2, generic_param1)
    }
    
    let mut pair = (1, true);
    let mut pair2 = (String::from("Radha"), true);
    println!("The reversed pair1 is {:?}", reverse(pair));

    // Here we are moving the pair to the reverse closure, but we can still access it later as below, HOW ??
    // ^ because our tuple is of type (i32, bool) both of which implement 'Copy' trait
    pair = (2, false);

    // However if our tuple had a String which does not implement 'Copy' trait we can not access the pair later
    println!("The reversed pair2 is {:?}", reverse_genric(pair2));
    // BELOW LINE WOULD GIVE ERROR: 'borrow of moved value: `pair2`'
    // ^ CANNOT ACCESS 'pair2' : print!("{:?}", pair2);

    // ^ To access 'pair2' again intialize it with some values then it would be fine !!
    pair2 = (String::from("Kanha"), true);
    // LIKE SO ..
    println!("{:?}", pair2);
    println!("{:?}", pair);

    //////////////////////////////////////////////////////////////////////////////
     
    // ^ USE OF '@' IN 'Match' Statements for Variable Binding & Pattern Mathcing Simultaneously !!

    fn age() -> i32 { 12 }

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // ^ HERE: 'n @ 1 ..= 12'
        // CHECKS IF A VALUE IS IN RANGE `1 - 12`` AND ASSIGNS IT TO 'n' IF EXISTS SIMULTANEOSULY, 
        // SO THAT WE CAN ACCESS IT FURTHER !!
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // ^ If we dont use '@' syntax then:
        1..=12 => println!("I'm a child"), // ^ You can't access the specific age here.
        n => println!("I'm an old person of age {:?}", n),
    }

    //////////////////////////////////////////////////////////////////////////////
    /// ^ GAURDS IN 'Match Statements'

    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // ^ The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    //////////////////////////////////////////////////////////////////////////////

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;

    // ^ I CANT JUST DO: 'if Foo::Bar == a' BEACUSE: 
    // ^ variants are not automatically comparable unless they implement the 'PartialEq' trait.
    // If i implment 'PartialEqu' Trait on the enum then we can directly compare without destructing using 'let'
    if let Foo::Bar = a {
        println!("a is foobar");
    }


}

// ^ USE OF 'dyn' KEYWORD !!

// ^ WE CANT JUST SPECIFY A TRAIT AS A RETURN TYPE IN RUST, 
// ^ BECAUSE RETURN TYPE OF A FUNCTION IN RUST MUST HAVE A CONCRETE TYPE,
// ^ SO AS A WORKAROUND WE CAN USE A 'Box' WITH 'dyn' KEYWORD: 
// ! EXPLANATION: box is just a reference to some memory in the heap. Because a reference has a statically-known size, and the compiler can guarantee it points to a heap-allocated Animal, we can return a trait from our function!

// The Rust compiler needs to know how much space every function's return type requires. 
// This means all your functions have to return a concrete type. Unlike other languages, 
// if you have a trait like Animal, you can't write a function that returns Animal, 
// because its different implementations will need different amounts of memory.

// However, there's an easy workaround. Instead of returning a trait object directly, 
// our functions return a Box which contains some Animal.

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}


// ! UN-UNDERSTANDLE RUST Behaviour:
// let mut count = 0;
// A closure to increment `count` could take either `&mut count` or `count`
// but `&mut count` is less restrictive so it takes that. Immediately
// borrows `count`.
//
// A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
// calling the closure mutates `count` which requires a `mut`.
// let mut inc = || {
//     count += 1;
//     println!("`count`: {}", count);
// };
// Call the closure using a mutable borrow.
// inc();
// The closure still mutably borrows `count` because it is called later.
// An attempt to reborrow will lead to an error.
// let _reborrow = &count; 
// ^ TODO: try uncommenting this line.
// inc();
// The closure no longer needs to borrow `&mut count`. Therefore, it is
// possible to reborrow without an error
// let _count_reborrowed = &mut count; 