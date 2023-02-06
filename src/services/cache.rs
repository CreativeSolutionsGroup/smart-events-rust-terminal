use std::{thread, time::Duration, env, sync::Arc};
use zmq::{Context, Socket, Message};
use crate::models::checkin::*;
use threadpool::ThreadPool;
use rusqlite::Connection;

// Create a new database or connect to an existing one
pub fn initialize_database() {
    // Tries to connect and if it can't connect then it will create a new database
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let create_table_str = 
        "CREATE TABLE IF NOT EXISTS check_ins (
            mac_address TEXT NOT NULL,
            student_id  TEXT NOT NULL UNIQUE,
            time_stamp  TEXT NOT NULL
        );";
    db.execute(create_table_str, ()).unwrap();
}

// Deletes a given checkin with id from the cache.
pub fn delete_check_in (id: &str) {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let delete_checkin_str = 
        format!("DELETE FROM check_ins WHERE student_id LIKE \"{}\";", id);
    db.execute(&delete_checkin_str, ()).unwrap();
}

// Inserts a checkin into the cache at the back
pub fn insert_check_in (check_in: &Checkin) {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let insert_checkin_str = 
        format!("INSERT INTO check_ins (mac_address, student_id, time_stamp) VALUES(\"{}\", \"{}\", \"{}\");", 
                &check_in.mac_address, &check_in.student_id, &check_in.time_stamp);
    match db.execute(&insert_checkin_str, ()) {
        Ok(_) => (),
        Err(_) => (),
    };
}

// Checks the cache at set intervals to send checkins
pub fn cache_observer() {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    // Get the proxy connection url
    let default_url: &str = "tcp://localhost:9951";
    let connection_url: Arc<String>;
    match env::var("PROXY_URL") {
        Ok(url) => connection_url = Arc::new(url),
        Err(_) => connection_url = Arc::new(default_url.to_owned())
    }

    loop {
        // Create pool of threads that wait till all threads finish
        let pool = ThreadPool::new(200);
        // Get all current check_ins in the cache
        let mut checkin_query = db.prepare("SELECT * FROM check_ins").unwrap();
        let checkin_map = checkin_query.query_map([], |row| {
            Ok(Checkin {
                mac_address: row.get(0).unwrap(),
                student_id: row.get(1).unwrap(),
                time_stamp: row.get(2).unwrap()
            })
        }).unwrap();

        for check_in in checkin_map {
            // Create a new thread for each checkin
            let temp_conn = connection_url.clone();
            pool.execute(move || {send_checkin(&check_in.unwrap(), temp_conn)});
        }
        pool.join();
        // Wait a few seconds before trying again
        thread::sleep(Duration::from_secs(3));
    }
}

// Send the checkin and delete once it is recieved
fn send_checkin(check_in: &Checkin, conn: Arc<String>) {
    // Send checkin over ZMQ
    let context: Context = zmq::Context::new();
    let proxy: Socket = context.socket(zmq::REQ).unwrap();
    match proxy.connect(&conn) {
        Ok(_) => (),
        Err(_) => return
    }
    
    let data = format!("checkin {} {} {}", check_in.mac_address, check_in.student_id, check_in.time_stamp);
    let mut msg: Message = zmq::Message::new();
    match proxy.send(data.as_bytes(), 0) {
        Ok(_) => {
            proxy.recv(&mut msg, 0).unwrap();
            if msg.as_str().unwrap().contains(&check_in.student_id) {
                println!("Sent checkin for student {}", check_in.student_id);
                delete_check_in(&check_in.student_id);
            }
        },
        Err(_) => {
            return
        }
    }
}