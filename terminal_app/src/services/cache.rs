use sqlx::{Sqlite, migrate::MigrateDatabase, SqlitePool};
use crate::models::checkin::*;

// Create a new database or connect to an existing one
pub async fn initialize_database(url: &str) {
    let db: SqlitePool;
    // Tries to connect and if it can't connect then it will create a new database
    match SqlitePool::connect(url).await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            Sqlite::create_database(url).await.unwrap();
            db = SqlitePool::connect(url).await.unwrap();
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
pub async fn delete_check_in (url: &str, id: &str) {
    let db: SqlitePool;
    match SqlitePool::connect(url).await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            initialize_database(url).await;
            db = SqlitePool::connect(url).await.unwrap();
        }
    }
    let delete_checkin_str = 
        format!("DELETE FROM check_ins WHERE student_id LIKE \"{}\";", id);
    let _result = sqlx::query(&delete_checkin_str).execute(&db).await;
    db.close().await;
}

// Inserts a checkin into the cache at the back
pub async fn insert_check_in (url: &str, check_in: &Checkin) {
    let db: SqlitePool;
    match SqlitePool::connect(url).await {
        Ok(pool) => { 
            db = pool;
        },
        Err(_) => {
            initialize_database(url).await;
            db = SqlitePool::connect(url).await.unwrap();
        }
    }
    let insert_checkin_str = 
        format!("INSERT INTO check_ins (mac_address, student_id) VALUES(\"{}\", \"{}\");", 
                &check_in.mac_address, &check_in.student_id);
    let _result = sqlx::query(&insert_checkin_str).execute(&db).await;
    db.close().await;
}