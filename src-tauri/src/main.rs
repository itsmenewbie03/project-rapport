// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod utils;

use utils::auth::is_credentials_valid;
use utils::auth::models::UserData;
use utils::db;

#[tauri::command]
async fn authenticate(email: &str, password: &str) -> Result<Option<UserData>, ()> {
    println!("[RUST]: authenticatation for {}:{}", email, password);
    db::init().await;
    if is_credentials_valid(email, password).await {
        // NOTE: we can safely unwrap this
        // as we do have valid credentials
        let user_data = db::get_user(email).await.unwrap();
        Ok(Some(UserData {
            name: user_data.name,
            id: user_data.id,
            email: user_data.email,
        }))
    } else {
        Ok(None)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![authenticate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
