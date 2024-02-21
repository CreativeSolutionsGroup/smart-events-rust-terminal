use chrono::Local;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use tokio::time::timeout;
use std::{collections::HashMap};
use std::time::Duration;

use crate::{models::checkin::Checkin, services::{cache::delete_many_check_ins, getid::get_booper_id}};

const URL: &str = "http://localhost:3001/api/sick";

pub async fn send_heartbeat() {
    let timestamp: String = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    let api_client = Client::new();
    let mut client_information = HashMap::new();
    client_information.insert("id", get_booper_id());
    client_information.insert("timestamp", timestamp.clone());

    // Make the request with a timeout of 2 seconds
    let response = timeout(Duration::from_secs(2), async {
        api_client
            .put(URL)
            .header(CONTENT_TYPE, "application/json")
            .json(&client_information)
            .send()
            .await
    }).await;

    match response {
        Ok(res) => {
            // Handle the response here
            match res {
                Ok(r) => println!("Request successful: {:?}", r),
                Err(e) => println!("Request failed with status: {:?}", e)
            }
        }
        Err(_) => {
            // Handle timeout or other errors here
            println!("Request timed out or encountered an error.");
        }
    }
}

pub async fn send_checkins(check_ins: HashMap<String, Checkin>) {
    let api_client = Client::new();
    
    match api_client.post(URL).json(&check_ins).send().await {
        Ok(x) => {
            println!(
                "Sent {} check ins -> {}",
                check_ins.len(),
                x.status().to_string()
            );
            delete_many_check_ins(check_ins);
        }
        Err(x) => println!("Sent no check ins -> {}", x),
    }
}
