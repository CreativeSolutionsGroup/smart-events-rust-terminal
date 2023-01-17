use mac_address::{MacAddress, get_mac_address};

pub fn get_mac() -> MacAddress {
    // gets the mac_address and returns it
    // If it can't be found or recieves an error we use the default
    let default_mac_address: MacAddress = MacAddress::new([0,0,0,0,0,0]);
    match get_mac_address() {
        Ok(Some(ma)) => { return ma },
        Ok(None) => { return default_mac_address },
        Err(_) => { return default_mac_address },
    }
}