mod models;
mod services;
use futures::executor::block_on;
use models::heartbeat::*;
use models::checkin::*;
use services::cache::*;

fn main() {
    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: "00:00:00:00:00:00".to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);

    // Example of cache
    // Use https://sqliteviewer.app/#/ to observe the sql table
    let db_url = "sqlite://cache.db";
    block_on(initialize_database(db_url));
    block_on(insert_check_in(db_url, &check_in));
    //block_on(delete_check_in(db_url, &check_in.student_id));
}