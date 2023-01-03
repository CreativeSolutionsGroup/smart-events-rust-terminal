mod models;
use mac_address::MacAddress;
use mac_address::get_mac_address;
use models::heartbeat::*;
use models::checkin::*;

fn get_mac() -> MacAddress {
    // gets the mac_address and returns it
    // If it can't be found or recieves an error we use the default
    let default_mac_address: MacAddress = MacAddress::new([0,0,0,0,0,0]);
    match get_mac_address() {
        Ok(Some(ma)) => { return ma },
        Ok(None) => { return default_mac_address },
        Err(_) => { return default_mac_address },
    }
}

fn main() {
    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);
}