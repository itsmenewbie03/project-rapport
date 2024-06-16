// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod utils;

use utils::auth::is_credentials_valid;
use utils::auth::models::UserData;
use utils::db;
use utils::jwt;
use utils::jwt::Claims;

use crate::utils::{faau, feedback::FeedbackData, recorder};

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

#[tauri::command]
async fn start_face_recording(id: &str) -> Result<String, String> {
    // NOTE: for now we just say every thing succeds
    println!("[RUST]: recording started for feedback_id: {}", id);
    match recorder::start(id).await {
        Ok(res) => Ok(res),
        Err(err) => {
            // NOTE: we can warn the user about the error
            utils::notifications::warn(&err);
            Err(err)
        }
    }
}

#[tauri::command]
async fn submit_feedback(id: &str, feedback: &str) -> Result<String, String> {
    recorder::stop().await;
    let feedback_data = FeedbackData::parse(feedback);
    match feedback_data {
        Ok(feedback_data) => {
            println!(
                "[RUST]: feedback submitted for feedback_id: {}\n[RUST]: feedback data: {:?}",
                id, feedback_data
            );
            // BUG: handle the Err variant
            faau::start(id).await.unwrap();
            feedback_data.save(id);
            Ok("Feedback submitted successfully".to_owned())
        }
        Err(e) => {
            println!("[RUST]: failed to parse feedback data: {}", e);
            Err("Failed to parse feedback data".to_owned())
        }
    }
}
#[tauri::command]
async fn clear_recording(id: &str) -> Result<String, String> {
    recorder::clear(id).await;
    Ok("Recording cleared!".to_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            authenticate,
            get_session,
            start_face_recording,
            submit_feedback,
            clear_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
