use chrono::Local;
use reqwest::Client;
use std::{collections::HashMap};

use crate::{models::checkin::Checkin, services::{cache::delete_many_check_ins, getid::get_booper_id}};

const URL: &str = "https://main.d3e17gvbrma8q6.amplifyapp.com/api/sick";

pub async fn send_heartbeat() {
    let timestamp: String = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    let api_client = Client::new();
    let mut client_information = HashMap::new();
    client_information.insert("id", get_booper_id());
    client_information.insert("timestamp", timestamp.clone());

    match api_client.put(URL).json(&client_information).send().await {
        Ok(x) => println!("Sent heartbeat at {} -> {}", timestamp, x.status().as_str()),
        Err(x) => println!("Failed sending heartbeat at {} -> {}", timestamp, x),
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
