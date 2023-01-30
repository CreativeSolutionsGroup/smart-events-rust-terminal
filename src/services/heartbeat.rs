use crate::{models::heartbeat::Heartbeat, services::get_mac::*};
use std::{time::Duration, thread, env};
use zmq::{Context, Socket, Message};

pub fn build_heartbeat() {
    let mut backoff = 0;
    let default_url: &str = "tcp://localhost:9951";
    loop {
        thread::sleep(Duration::from_secs(backoff));
        
        let context: Context = zmq::Context::new();
        let proxy: Socket = context.socket(zmq::REQ).unwrap();
        
        let connection_url: String;
        match env::var("PROXY_URL") {
            Ok(url) => connection_url = url,
            Err(_) => connection_url = default_url.to_string()
        }
        
        loop {
            match proxy.connect(&connection_url) {
                Ok(_) => println!("ZMQ Connected"),
                Err(_) => {
                    backoff *= 2;
                    if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                    println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                    break;
                }
            }
            
            let client: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
            let data = format!("heartbeat {}", client.mac_address);
            let mut msg: Message = zmq::Message::new();
            match proxy.send(data.as_bytes(), 0) {
                Ok(_) => {
                    proxy.recv(&mut msg, 0).unwrap();
                    if msg.as_str().unwrap().contains(&client.mac_address) {
                        println!("sent heartbeat");
                    }
                },
                Err(_) => {
                    backoff *= 2;
                    if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                    println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                    break;
                }
            }
            proxy.disconnect(&connection_url).unwrap();
            thread::sleep(Duration::from_secs(10));
            backoff = 0;
        }
    }
}