use std::{thread, time::Duration, collections::HashMap};
use crate::models::checkin::*;
use rusqlite::Connection;

use super::apiclient::send_checkins;

// Create a new database or connect to an existing one
pub fn initialize_database() {
    // Tries to connect and if it can't connect then it will create a new database
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let create_checkin_table_str = 
        "CREATE TABLE IF NOT EXISTS check_ins (
            id          TEXT        NOT NULL,
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

pub fn delete_many_check_ins (check_ins: HashMap<String, Checkin>) {
    for check_in in check_ins {
        delete_check_in(&check_in.0);
    }
}

// Inserts a checkin into the cache at the back
pub fn insert_check_in (check_in: &Checkin) {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    let insert_checkin_str = 
        format!("INSERT INTO check_ins (id, student_id, time_stamp) VALUES(\"{}\", \"{}\", \"{}\");", 
                &check_in.id, &check_in.student_id, &check_in.time_stamp);
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
pub async fn cache_observer() {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();
    
    loop {
        // Get all current check_ins in the cache
        let mut checkin_query = db.prepare("SELECT * FROM check_ins").unwrap();
        let checkin_map = checkin_query.query_map([], |row| {
            Ok(Checkin {
                id: row.get(0).unwrap(),
                student_id: row.get(1).unwrap(),
                time_stamp: row.get(2).unwrap()
            })
        }).unwrap();

        let mut check_ins: HashMap<String, Checkin> = HashMap::new();

        for check_in in checkin_map {
            let c = check_in.unwrap();
            check_ins.insert(c.id.clone(), c);
        }
        send_checkins(check_ins).await;
        // Wait a few seconds before trying again
        thread::sleep(Duration::from_secs(3));
    }
}

// Sends the errors we have recieved over ZMQ
pub fn error_observer() {
    let db: Connection = Connection::open("./cache.sqlite").unwrap();

    loop {
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

        let mut errors: HashMap<Option<u64>, AppError> = HashMap::new();

        for app_error in error_map {
            let e = app_error.unwrap();
            errors.insert(e.id, e);
        }
        // Wait a few seconds before trying again
        thread::sleep(Duration::from_secs(3));
    }
}