mod models;
mod services;
use services::{cache::*, input::*, heartbeat::*};
use futures::executor::block_on;
use std::{thread, sync::{Arc, Mutex}, time::Duration};

fn main() {
    // Mutex required for Windows as heartbeat and sqlite try to use the same memory location at the same time
    let mutex_lock = Arc::new(Mutex::new(0));
    let heartbeat_arc = mutex_lock.clone();
    let main_arc = mutex_lock.clone();
    
    // Initialize the database we will be using
    block_on(initialize_database(main_arc.clone()));
    
    // This creates a new thread that contains the heartbeat
    // let heartbeat_handle = thread::spawn(move || {build_heartbeat(heartbeat_arc.clone())});

    // Start up the waiting for input
    let input_handle = thread::spawn(move || {wait_for_input(main_arc.clone())});

    // Threads must be joined back in or when main exits, it will force close any extra threads
    // heartbeat_handle.join().unwrap();
    input_handle.join().unwrap();
}