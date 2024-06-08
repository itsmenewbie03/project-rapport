// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod utils;

use utils::auth::is_credentials_valid;
use utils::auth::models::UserData;
use utils::db;
use utils::jwt;
use utils::jwt::Claims;

#[tauri::command]
async fn authenticate(email: &str, password: &str) -> Result<Option<String>, ()> {
    println!("[RUST]: authentication for {}:{}", email, password);
    db::init().await;
    if is_credentials_valid(email, password).await {
        // NOTE: we can safely unwrap this
        // as we do have valid credentials
        let user_data = db::get_user(email).await.unwrap();
        let claims = Claims {
            id: user_data.id,
            name: user_data.name,
            email: user_data.email,
            exp: jwt::get_exp() as usize,
        };
        let token = jwt::generate(claims);
        match token {
            Some(token) => Ok(Some(token)),
            None => {
                println!("[RUST]: rust failed to generate jwt");
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

#[tauri::command]
async fn get_session(token: Option<&str>) -> Result<Option<UserData>, ()> {
    match token {
        Some(token) => {
            println!("[RUST]: validating {}", token);
            match jwt::validate(token) {
                Some(token_data) => Ok(Some(UserData {
                    id: token_data.id,
                    name: token_data.name,
                    email: token_data.email,
                })),
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![authenticate, get_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
