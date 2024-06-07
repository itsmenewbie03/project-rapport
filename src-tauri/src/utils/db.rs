use crate::utils::auth::models::User;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
const DB_URL: &str = "sqlite://sqlite.db";

async fn get_db_connection() -> Result<sqlx::Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect(DB_URL).await
}

/// Since this is a core part of the system, we will just let this panic
/// when the database fails to initialize.
pub async fn init() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("[RUST]: Fresh database created!");
        Sqlite::create_database(DB_URL).await.unwrap();
    }

    let db = get_db_connection().await.unwrap();
    // NOTE: weird but this doesn't return any indication that the table is indeed created
    let _result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
                id       INTEGER PRIMARY KEY NOT NULL,
                name     VARCHAR(250) NOT NULL,
                email    VARCHAR(250) NOT NULL UNIQUE,
                password VARCHAR(250) NOT NULL
        );",
    )
    .execute(&db)
    .await
    .unwrap();
    // INFO: we will now check if any user exists
    let user_results = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();
    // NOTE: we don't have any users so we will create the default user
    if user_results.is_empty() {
        println!("[RUST]: fresh user table created!");
        let result = sqlx::query("INSERT INTO users (name,email,password) VALUES (?,?,?)")
            .bind("Project Rapport Admin")
            .bind("admin@projectrapport.rs")
            .bind("$2a$10$c/A.2lQ0RrQ.bzTPtPxJteQ9N/1eDLhaJAfi24v.xt7Y5YR.gxiCu")
            .execute(&db)
            .await
            .unwrap();
        if result.rows_affected() == 0 {
            // INFO: since we failed to create an admin
            // let's crash the system by calling panic macro
            // after dropping the database
            Sqlite::drop_database(DB_URL).await.unwrap();
            panic!("Failed to create user!\nDatabase Dropped!");
        }
        println!("[RUST]: default admin created!");
        println!("[RUST]: Fresh database initialization completed!");
        // NOTE: we can get rid of the return by ommitting the semi-colon on the line above
        // but that way is kinda hard to tell the we're doing an early return
        return;
    }
    println!("[RUST]: Database initialization completed!");
}

pub async fn get_user(email: &str) -> Option<User> {
    let db = get_db_connection().await.unwrap();
    let user_results = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=(?)")
        .bind(email)
        .fetch_all(&db)
        .await
        .unwrap();
    if user_results.is_empty() {
        // INFO: no user is found
        return None;
    }
    // NOTE: emails has UNIQUE constraint so we can safely
    // return a clone of the first result
    Some(user_results[0].clone())
}
