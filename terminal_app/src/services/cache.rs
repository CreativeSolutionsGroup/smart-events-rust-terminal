use sqlx::Connection;

// Create a new database
pub fn initialize_database() -> SqliteConnection {
    SqliteConnection::conect("sqlite::memory:").await?;
}

// Deletes a given checkin with id from the cache.
pub fn delete_check_in (db: SqliteConnection, id: String) {
    db.execute(sqlx::query("DELETE FROM ")).await?;
}

// Inserts a checkin into the cache at the back
pub fn insert_check_in (db: SqliteConnection, check_in: Checkin) {
    db
}