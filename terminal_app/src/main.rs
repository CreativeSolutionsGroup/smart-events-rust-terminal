mod models;
mod services;
use std::i128;
use std::io;

use mac_address::MacAddress;
use mac_address::get_mac_address;
use futures::executor::block_on;
use models::heartbeat::*;
use models::checkin::*;
use services::cache::*;

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

fn wait_for_input() {
    println!("Input ID:");
    let id_length = 5;
    let mut id = String::new();
 
    io::stdin().read_line(&mut id).expect("failed to readline");
    id = id.trim().to_string();

    if id.len() == id_length {
        match id.parse::<i128>() {
            Ok(_) => {
                // correct id parameters
                let new_checkin: Checkin = Checkin { mac_address: get_mac().to_string(), student_id: id.trim().to_string() };
                println!("mac_address: {}, student_id: {}", new_checkin.mac_address, new_checkin.student_id);
                block_on(insert_check_in(&new_checkin));
            },
            Err(e) => println!("Did not have a correct student id. Recieved: {}", e),
        }
    } else if id.len() > id_length {
        let mod_id = &id[0..id_length];
        match mod_id.parse::<i128>() {
            Ok(_) => {
                // correct id parameters
                let new_checkin: Checkin = Checkin { mac_address: get_mac().to_string(), student_id: mod_id.trim().to_string() };
                println!("mac_address: {}, student_id: {}", new_checkin.mac_address, new_checkin.student_id);
                block_on(insert_check_in(&new_checkin));
            },
            Err(e) => println!("Did not have a correct student id. Recieved: {}", e),
        }
    }
}

fn main() {
    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);
    
    // Example of cache
    // Use https://sqliteviewer.app/#/ to observe the sql table
    block_on(initialize_database());
    // block_on(delete_check_in(&check_in.student_id));

    wait_for_input();
}