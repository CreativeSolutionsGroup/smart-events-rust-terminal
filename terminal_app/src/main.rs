mod models;
mod services;
use services::{input::*, heartbeat::*, cache::initialize_database};
use std::thread;

fn main() {
    initialize_database();

    // This creates a new thread that contains the heartbeat
    let heartbeat_handle = thread::spawn(|| {build_heartbeat()});

    // Start up the waiting for input
    let input_handle = thread::spawn(|| {wait_for_input()});

    // Threads must be joined back in or when main exits, it will force close any extra threads
    heartbeat_handle.join().unwrap();
    input_handle.join().unwrap();
}