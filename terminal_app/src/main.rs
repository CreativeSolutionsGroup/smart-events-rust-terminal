mod models;
mod services;
use services::cache::*;
use zmq::{Context, Socket};
use futures::executor::block_on;
use std::{env, thread, time::Duration, sync::{Arc, Mutex}};
use models::{heartbeat::*, checkin::*};
use mac_address::{MacAddress, get_mac_address};

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

fn build_heartbeat(lock: Arc<Mutex<i8>>) {
    let mut backoff = 0;
    let default_url: &str = "tcp://localhost:9950";
    loop {
        thread::sleep(Duration::from_secs(backoff));
        
        let context: Context = zmq::Context::new();
        let proxy: Socket = context.socket(zmq::REQ).unwrap();
        
        let connection_url: String;
        match env::var("heartbeat_url") {
            Ok(url) => connection_url = url,
            Err(_) => connection_url = default_url.to_string()
        }
        
        loop {
            let mutex_lock = lock.lock().unwrap();
            match proxy.connect(&connection_url) {
                Ok(_) => println!("ZMQ Connected"),
                Err(_) => {
                    backoff *= 2;
                    if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                    println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                    std::mem::drop(mutex_lock);
                    break;
                }
            }
            
            let client: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
            let data = format!("heartbeat {}", client.mac_address);
            match proxy.send(data.as_bytes(), 0) {
                Ok(_) => println!("sent heartbeat"),
                Err(_) => {
                    backoff *= 2;
                    if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                    println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                    std::mem::drop(mutex_lock);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(10));
            backoff = 0;
        }
    }
}

fn main() {
    // Mutex required for Windows as heartbeat and sqlite try to use the same memory location at the same time
    let mutex_lock = Arc::new(Mutex::new(0));
    let heartbeat_arc = mutex_lock.clone();
    let main_arc = mutex_lock.clone();
    
    // This creates a new thread that contains the heartbeat
    let heartbeat_handle = thread::spawn(move || {build_heartbeat(heartbeat_arc.clone())});

    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);
    
    // Example of cache
    // Use https://sqliteviewer.app/#/ to observe the sql table
    let db_url = "sqlite://cache.db";
    block_on(initialize_database(db_url, main_arc.clone()));
    block_on(insert_check_in(db_url, &check_in, main_arc.clone()));
    block_on(delete_check_in(db_url, &check_in.student_id, main_arc.clone()));

    // Threads must be joined back in or when main exits, it will force close any extra threads
    let _heartbeat_res = heartbeat_handle.join();
}
