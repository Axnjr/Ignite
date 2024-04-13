pub fn get_request_limit_from_plan_name(val: &str) -> i64 {
    match val {
        "Hobby" => 500,
        "Pro" => 1000000,
        "StartUp" => 5000000,
        _ => 0,
    }
}

pub fn get_connection_limit_from_plan_name(val: &str) -> i64 {
    match val {
        "Hobby" => 10,
        "Pro" => 500,
        "StartUp" => 2000,
        _ => 0,
    }
}