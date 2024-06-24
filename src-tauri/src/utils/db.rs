use std::collections::HashMap;

use crate::utils::{auth::models::User, models::ConfigData};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::runtime::Runtime;

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

    // INFO: create trad_feedback_data table
    let _result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS trad_feedback_data (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                data       TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    // INFO: create hybrid_feedback_data table
    let _result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS hybrid_feedback_data (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                data       TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    // INFO: create configs table
    let _result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS configs (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL UNIQUE,
                value TEXT
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

pub async fn get_configs() -> Option<Vec<ConfigData>> {
    let db = get_db_connection().await.unwrap();
    let configs = sqlx::query_as::<_, ConfigData>("SELECT * FROM configs;")
        .fetch_all(&db)
        .await
        .unwrap();
    if configs.is_empty() {
        return None;
    }
    Some(configs)
}

pub async fn save_configs(config: HashMap<String, String>) -> bool {
    let db = get_db_connection().await.unwrap();
    for (key, value) in config.iter() {
        let result = sqlx::query("INSERT OR REPLACE INTO configs (name, value) VALUES (?, ?)")
            .bind(key)
            .bind(value)
            .execute(&db)
            .await;
        if result.is_err() {
            return false;
        }
    }
    true
}

pub async fn update_profile(email: &str, data: HashMap<String, String>) -> bool {
    let db = get_db_connection().await.unwrap();
    let user = get_user(email).await;
    match user {
        Some(_) => {
            let mut update_data: Vec<String> = vec![];
            data.iter().for_each(|(key, value)| {
                update_data.push(format!("{} = '{}'", key, value));
            });
            let query = format!(
                "UPDATE users SET {} WHERE email = (?)",
                update_data.join(", "),
            );
            let result = sqlx::query(&query).bind(email).execute(&db).await;
            match result {
                Ok(res) => res.rows_affected() == 1,
                Err(_) => false,
            }
        }
        None => false,
    }
}

pub async fn change_password(email: &str, new_password: &str) -> bool {
    let db = get_db_connection().await.unwrap();
    let password_hash = bcrypt::hash(new_password, 10).unwrap();
    let result = sqlx::query("UPDATE users SET password = (?) WHERE email = (?)")
        .bind(password_hash)
        .bind(email)
        .execute(&db)
        .await;
    match result {
        Ok(res) => res.rows_affected() == 1,
        Err(_) => false,
    }
}

pub async fn save_trad_feedback(data: &str) -> bool {
    let db = get_db_connection().await.unwrap();
    let result = sqlx::query("INSERT INTO trad_feedback_data (data) VALUES (?)")
        .bind(data)
        .execute(&db)
        .await;
    match result {
        Ok(res) => res.rows_affected() == 1,
        Err(_) => false,
    }
}

pub async fn save_hybrid_feedback(data: &str) -> bool {
    let db = get_db_connection().await.unwrap();
    let result = sqlx::query("INSERT INTO hybrid_feedback_data (data) VALUES (?)")
        .bind(data)
        .execute(&db)
        .await;
    match result {
        Ok(res) => res.rows_affected() == 1,
        Err(_) => false,
    }
}

pub fn save_hybrid_feedback_sync(data: &str) -> bool {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let db = get_db_connection().await.unwrap();
        let result = sqlx::query("INSERT INTO hybrid_feedback_data (data) VALUES (?)")
            .bind(data)
            .execute(&db)
            .await;
        match result {
            Ok(res) => res.rows_affected() == 1,
            Err(_) => false,
        }
    })
}
