// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::auth::User;

pub mod auth;

#[tauri::command]
async fn authenticate(email: &str, password: &str) -> Result<User, ()> {
    println!("[RUST]: authenticatation for {}:{}", email, password);
    // TODO: perform logic
    Ok(User {
        name: "John Doe".to_owned(),
        id: 1,
        email: "johndoe@example.com".to_owned(),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![authenticate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
