use crate::{models::heartbeat::Heartbeat, services::get_mac::*};
use std::{time::Duration, thread, env};
use zmq::{Context, Socket, Message};

pub fn build_heartbeat() {
    let mut backoff = 0;
    let default_url: &str = "tcp://localhost:9951";
    let connection_url: String;
    match env::var("PROXY_URL") {
        Ok(url) => connection_url = url,
        Err(_) => connection_url = default_url.to_string()
    }
    println!("Proxy URL: {}", connection_url);

    loop {
        thread::sleep(Duration::from_secs(backoff));
        
        let context: Context = zmq::Context::new();
        let proxy: Socket = context.socket(zmq::REQ).unwrap();
        match proxy.set_rcvtimeo(2000) {
            Ok(_) => {},
            Err(_) => {
                backoff *= 2;
                if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                continue;
            }
        }
        
        match proxy.connect(&connection_url) {
            Ok(_) => {},
            Err(_) => {
                backoff *= 2;
                if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                continue;
            }
        }

        loop {
            let client: Heartbeat = Heartbeat { mac_address: get_mac().to_string() };
            let data = format!("heartbeat {}", client.mac_address);
            let mut msg: Message = zmq::Message::new();
            match proxy.send(data.as_bytes(), 0) {
                Ok(_) => {
                    match proxy.recv(&mut msg, 0) {
                        Ok(_) => {
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
                },
                Err(_) => {
                    backoff *= 2;
                    if backoff > 128 { backoff = 128 } else if backoff == 0 { backoff = 1 }
                    println!("ZMQ Error. Attempting to reconnect in {} seconds", backoff);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(10));
            backoff = 0;
        }
    }
}