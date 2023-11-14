mod models;
mod services;
use services::{input::*, heartbeat::*, cache::{initialize_database, cache_observer}, getid::*};

#[tokio::main]
async fn main() {
    initialize_database();
    println!("Booper ID: {}", get_booper_id());
 
    let heartbeat_handle = build_heartbeat();
    let input_handle = wait_for_input();
    let cache_send_handle = cache_observer();
    
    futures::join!(heartbeat_handle, input_handle, cache_send_handle);
}