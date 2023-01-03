mod models;
mod services;
use std::env;
use async_recursion::async_recursion;
use std::{thread, time::Duration};
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

#[async_recursion(?Send)]
async fn build_heartbeat(mut backoff: u64) {
    thread::sleep(Duration::from_secs(backoff));
    
    let default_url: &str = "tcp://localhost:3001";
    let context = zmq::Context::new();
    let p = context.socket(zmq::PUB).unwrap();
    
    let connection_url: String;
    match env::var("heartbeat_url") {
        Ok(url) => connection_url = url,
        Err(_) => connection_url = default_url.to_string()
    }

    match p.connect(&connection_url) {
        Ok(_) => (),
        Err(_) => {
            backoff *= 2;
            if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
            println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
            build_heartbeat(backoff).await;
        }
    }

    let client: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
    
    loop {
        println!("sending");
        p.send(&client.mac_address, 0).unwrap();
        thread::sleep(Duration::from_secs(10));
        backoff = 0;
    }
}

fn main() {
    build_heartbeat(0);

    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);
    
    // Example of cache
    // Use https://sqliteviewer.app/#/ to observe the sql table
    let db_url = "sqlite://cache.db";
    block_on(initialize_database(db_url));
    block_on(insert_check_in(db_url, &check_in));
    block_on(delete_check_in(db_url, &check_in.student_id));
}