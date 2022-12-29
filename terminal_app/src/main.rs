mod models;
use models::heartbeat::*;
use models::checkin::*;

fn main() {
    // Just a small example of how to use the models
    let beat: Heartbeat = Heartbeat { mac_address: "00:00:00:00:00:00".to_string() };
    let check_in: Checkin = Checkin { mac_address: beat.mac_address, student_id: "12345".to_string() };
    println!("mac_address: {}, student_id: {}", check_in.mac_address, check_in.student_id);
}