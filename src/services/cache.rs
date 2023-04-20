use std::{thread, time::Duration, env, sync::Arc};
use zmq::{Context, Socket, Message};
use crate::models::checkin::*;
use threadpool::ThreadPool;
use rusqlite::Connection;

// Create a new database or connect to an existing one
pub fn initialize_database() {
    // Tries to connect and if it can't connect then it will create a new database
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let create_checkin_table_str = 
        "CREATE TABLE IF NOT EXISTS check_ins (
            mac_address TEXT        NOT NULL,
            student_id  TEXT        NOT NULL UNIQUE,
            time_stamp  TEXT        NOT NULL
        );";
    let create_error_table_str = 
        "CREATE TABLE IF NOT EXISTS errors (
            id          INTEGER     PRIMARY KEY,
            type        TEXT        NOT NULL,
            input       TEXT        NOT NULL,
            time_stamp  TEXT        NOT NULL,
            received    INTEGER     NOT NULL
        );";
    db.execute(create_checkin_table_str, ()).unwrap();
    db.execute(create_error_table_str, ()).unwrap();
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

// Inserts an error into the error table
pub fn save_error (app_error: &AppError) {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let insert_error_str = 
        format!("INSERT INTO errors (type, input, time_stamp, received) VALUES(\"{}\", \"{}\", \"{}\", {});", 
                &app_error.etype, &app_error.input, &app_error.time, &app_error.received);
    match db.execute(&insert_error_str, ()) {
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
    match proxy.set_rcvtimeo(2000) {
        Ok(_) => {},
        Err(_) => return
    }
    match proxy.connect(&conn) {
        Ok(_) => (),
        Err(_) => return
    }
    
    let data = format!("checkin {} {} {}", check_in.mac_address, check_in.student_id, check_in.time_stamp);
    let mut msg: Message = zmq::Message::new();
    match proxy.send(data.as_bytes(), 0) {
        Ok(_) => {
            match proxy.recv(&mut msg, 0) {
                Ok(_) => {
                    if msg.as_str().unwrap().contains(&check_in.student_id) {
                        println!("Sent checkin for student {}", check_in.student_id);
                        delete_check_in(&check_in.student_id);
                    }
                },
                Err(_) => return
            }
        },
        Err(_) => {
            return
        }
    }
}

// Sends the errors we have recieved over ZMQ
pub fn error_observer() {
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
        let mut error_query = db.prepare("SELECT * FROM errors WHERE received = 0").unwrap();
        let error_map = error_query.query_map([], |row| {
            Ok(AppError {
                id: row.get(0).unwrap(),
                etype: row.get(1).unwrap(),
                input: row.get(2).unwrap(),
                time: row.get(3).unwrap(),
                received: row.get(4).unwrap()
            })
        }).unwrap();

        for app_error in error_map {
            if app_error.as_ref().unwrap().received == 1 { continue; }
            // Create a new thread for each checkin
            let temp_conn = connection_url.clone();
            pool.execute(move || {send_error(&app_error.unwrap(), temp_conn)});
        }
        pool.join();
        // Wait a few seconds before trying again
        thread::sleep(Duration::from_secs(3));
    }
}

// Send the error and mark as delivered once it is recieved
fn send_error(app_error: &AppError, conn: Arc<String>) {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    // Send checkin over ZMQ
    let context: Context = zmq::Context::new();
    let proxy: Socket = context.socket(zmq::REQ).unwrap();
    match proxy.set_rcvtimeo(2000) {
        Ok(_) => {},
        Err(_) => return
    }
    match proxy.connect(&conn) {
        Ok(_) => (),
        Err(_) => return
    }
    
    let data = format!("error {} {} {}", app_error.etype, app_error.input, app_error.time);
    let mut msg: Message = zmq::Message::new();
    match proxy.send(data.as_bytes(), 0) {
        Ok(_) => {
            match proxy.recv(&mut msg, 0) {
                Ok(_) => {
                    if msg.as_str().unwrap().contains(&format!("{} {}", &app_error.etype, app_error.input)) {
                        let update_error_str = 
                            format!("UPDATE errors SET received = 1 WHERE id = {:?};", 
                                &app_error.id);
                        match db.execute(&update_error_str, ()) {
                            Ok(_) => (),
                            Err(_) => (),
                        };
                    }
                },
                Err(_) => return
            };
        },
        Err(_) => {
            return
        }
    }
}