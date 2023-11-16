use std::env;

pub fn get_booper_id() -> String {
    return match env::var("BOOPER_ID") {
        Ok(got_id) => got_id,
        Err(_) => String::from("DEV"),
    }
}