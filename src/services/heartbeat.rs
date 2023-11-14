use std::{thread, time::Duration};
use crate::services::apiclient::send_heartbeat;

pub async fn build_heartbeat() {
    loop {
        send_heartbeat().await;

        thread::sleep(Duration::from_secs(10));
    }
}
