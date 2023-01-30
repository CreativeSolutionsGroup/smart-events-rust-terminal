mod models;
mod services;
use services::{input::*, heartbeat::*, cache::{initialize_database, cache_observer}};
use std::thread;

fn main() {
    initialize_database();

    // Startup necessary threads
    let heartbeat_handle = thread::spawn(|| {build_heartbeat()});
    let input_handle = thread::spawn(|| {wait_for_input()});
    let chache_sender_handle = thread::spawn(|| {cache_observer()});

    // Threads must be joined back in or when main exits, it will force close any extra threads
    heartbeat_handle.join().unwrap();
    input_handle.join().unwrap();
    chache_sender_handle.join().unwrap();
}