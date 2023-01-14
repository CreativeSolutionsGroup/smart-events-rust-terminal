use sqlx::{Sqlite, migrate::MigrateDatabase, SqlitePool};
use std::sync::{Arc, Mutex};
use crate::models::checkin::*;

// Create a new database or connect to an existing one
pub async fn initialize_database(lock: Arc<Mutex<i8>>) {
    let db: SqlitePool;
    // Tries to connect and if it can't connect then it will create a new database
    let _mutex_lock = lock.lock().unwrap();
    match SqlitePool::connect("sqlite://cache.db").await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            Sqlite::create_database("sqlite://cache.db").await.unwrap();
            db = SqlitePool::connect("sqlite://cache.db").await.unwrap();
        }
    }
    let create_table_str = 
        "CREATE TABLE IF NOT EXISTS check_ins (
            mac_address TEXT NOT NULL,
            student_id  TEXT NOT NULL UNIQUE
        );";
    let _result = sqlx::query(create_table_str).execute(&db).await;
    db.close().await;
}

// Deletes a given checkin with id from the cache.
pub async fn delete_check_in (id: &str, lock: Arc<Mutex<i8>>) {
    let db: SqlitePool;
    let mutex_lock = lock.lock().unwrap();
    match SqlitePool::connect("sqlite://cache.db").await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            std::mem::drop(mutex_lock);
            initialize_database(lock).await;
            db = SqlitePool::connect("sqlite://cache.db").await.unwrap();
        }
    }
    let delete_checkin_str = 
        format!("DELETE FROM check_ins WHERE student_id LIKE \"{}\";", id);
    let _result = sqlx::query(&delete_checkin_str).execute(&db).await;
    db.close().await;
}

// Inserts a checkin into the cache at the back
pub async fn insert_check_in (check_in: &Checkin, lock: Arc<Mutex<i8>>) {
    let db: SqlitePool;
    let mutex_lock = lock.lock().unwrap();
    match SqlitePool::connect("sqlite://cache.db").await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            std::mem::drop(mutex_lock);
            initialize_database(lock).await;
            db = SqlitePool::connect("sqlite://cache.db").await.unwrap();
        }
    }
    let insert_checkin_str = 
        format!("INSERT INTO check_ins (mac_address, student_id) VALUES(\"{}\", \"{}\");", 
                &check_in.mac_address, &check_in.student_id);
    let _result = sqlx::query(&insert_checkin_str).execute(&db).await;
    db.close().await;
}