use rusqlite::Connection;
use crate::models::checkin::*;

// Create a new database or connect to an existing one
pub fn initialize_database() {
    // Tries to connect and if it can't connect then it will create a new database
    let db: Connection = Connection::open("./cache.db").unwrap();
    let create_table_str = 
        "CREATE TABLE IF NOT EXISTS check_ins (
            mac_address TEXT NOT NULL,
            student_id  TEXT NOT NULL UNIQUE
        );";
    db.execute(create_table_str, ()).unwrap();
}

// Deletes a given checkin with id from the cache.
pub fn delete_check_in (id: &str) {
    let db: Connection = Connection::open("./cache.db").unwrap();
    let delete_checkin_str = 
        format!("DELETE FROM check_ins WHERE student_id LIKE \"{}\";", id);
    db.execute(&delete_checkin_str, ()).unwrap();
}

// Inserts a checkin into the cache at the back
pub fn insert_check_in (check_in: &Checkin) {
    let db: Connection = Connection::open("./cache.db").unwrap();
    let insert_checkin_str = 
        format!("INSERT INTO check_ins (mac_address, student_id) VALUES(\"{}\", \"{}\");", 
                &check_in.mac_address, &check_in.student_id);
    db.execute(&insert_checkin_str, ()).unwrap();
}