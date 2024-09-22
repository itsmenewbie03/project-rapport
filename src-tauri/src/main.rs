// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod utils;

use std::collections::HashMap;
use utils::auth::is_credentials_valid;
use utils::auth::models::UserData;
use utils::db;
use utils::db::archive_feedbacks;
use utils::feedback::TradFeedbackData;
use utils::jwt;
use utils::jwt::Claims;
use utils::models::DateRangeFilter;
use utils::models::ServiceData;

use crate::utils::{
    alerts, faau,
    feedback::{FeedbackData, FeedbackType, HybridFeedbackData},
    mailer,
    models::ConfigData,
    notifications, recorder, reports,
};

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
    match faau::start(id).await {
        Ok(res) => Ok(res),
        Err(err) => {
            // NOTE: we can warn the user about the error
            utils::notifications::warn(&err);
            Err(err)
        }
    }
}

#[tauri::command]
async fn take_photo(id: &str, quality: &str) -> Result<String, String> {
    println!("[RUST]: take photo for {} for feedback {}", quality, id);
    match faau::capture(id, quality).await {
        Ok(res) => Ok(res),
        Err(err) => {
            // NOTE: we can warn the user about the error
            utils::notifications::warn(&err);
            Err(err)
        }
    }
}

#[tauri::command]
async fn submit_feedback(
    id: &str,
    feedback: &str,
    emotion: Option<String>,
    metadata: &str,
    recording: bool,
) -> Result<String, String> {
    let feedback_data = FeedbackData::parse(feedback);
    match feedback_data {
        Ok(feedback_data) => {
            println!(
                "[RUST]: feedback submitted for feedback_id: {}\n[RUST]: feedback data: {:?}",
                id, feedback_data
            );
            // BUG: handle the Err variant
            if !recording {
                // NOTE: when the user did not allowed recording
                // we have nothing lef to do but store the feedback data
                println!("[RUST]: feedback submitted without recording will save to trad!");
                let feedback_mean = feedback_data.mean();
                // NOTE: adapted from
                // 5.   4.21 - 5.00   VS
                // 4.   3.41 - 4.20   S
                // 3.   2.61 - 3.40   NEUTRAL
                // 2.   1.81 - 2.60   D
                // 1.   1.00 - 1.80   VD
                let feedback_category = if feedback_mean >= 3.41 {
                    "positive"
                } else if feedback_mean < 2.61 {
                    "negative"
                } else {
                    "neutral"
                };

                let metadata: HashMap<String, String> = serde_json::from_str(metadata).unwrap();
                let trad_feedback_data = TradFeedbackData {
                    feedback_data,
                    metadata,
                };
                db::save_trad_feedback(
                    &serde_json::to_string(&trad_feedback_data).unwrap(),
                    feedback_category,
                )
                .await;
                return Ok("Feedback submitted successfully".to_owned());
            };
            // INFO: we will only need to stop recording
            // and start faau if the user allowed recording
            recorder::stop().await;
            if let Some(emotion) = emotion {
                let feedback_mean = feedback_data.mean();
                // NOTE: adapted from
                // 5.   4.21 - 5.00   VS
                // 4.   3.41 - 4.20   S
                // 3.   2.61 - 3.40   NEUTRAL
                // 2.   1.81 - 2.60   D
                // 1.   1.00 - 1.80   VD
                let feedback_category = if feedback_mean >= 3.41 {
                    "positive"
                } else if feedback_mean < 2.61 {
                    "negative"
                } else {
                    "neutral"
                };

                // NOTE: email alert feature
                alerts::check_threshold(feedback_category).await;

                let mut metadata: HashMap<String, String> = serde_json::from_str(metadata).unwrap();
                metadata.insert("feedback_category".to_owned(), feedback_category.to_owned());
                let hybrid_feedback_data = HybridFeedbackData {
                    feedback_data,
                    emotion_data: serde_json::from_str(&emotion).unwrap(),
                    metadata,
                };
                db::save_hybrid_feedback(
                    &serde_json::to_string(&hybrid_feedback_data).unwrap(),
                    feedback_category,
                )
                .await;
            }
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

#[tauri::command]
async fn get_configs() -> Result<Vec<ConfigData>, String> {
    match db::get_configs().await {
        Some(configs) => Ok(configs),
        None => Err("No configs set yet!".to_owned()),
    }
}

#[tauri::command]
async fn get_services() -> Result<Vec<ServiceData>, String> {
    match db::get_services().await {
        Some(services) => Ok(services),
        None => Err("No services added yet!".to_owned()),
    }
}

#[tauri::command]
async fn add_service(service: &str) -> Result<String, String> {
    if db::add_service(service).await {
        Ok("Service added successfully!".to_owned())
    } else {
        Err("Failed to add service!".to_owned())
    }
}

#[tauri::command]
async fn delete_service(service: &str) -> Result<String, String> {
    if db::delete_service(service).await {
        Ok("Service delete successfully!".to_owned())
    } else {
        Err("Failed to delete service!".to_owned())
    }
}

#[tauri::command]
async fn edit_service(target: &str, update: &str) -> Result<String, String> {
    if db::edit_service(target, update).await {
        Ok("Service updated successfully!".to_owned())
    } else {
        Err("Failed to update service!".to_owned())
    }
}

#[tauri::command]
async fn save_configs(data: HashMap<String, String>) -> Result<String, String> {
    if db::save_configs(data).await {
        Ok("Config saved successfully".to_owned())
    } else {
        Err("Failed to save config".to_owned())
    }
}

#[tauri::command]
async fn update_profile(email: &str, data: HashMap<String, String>) -> Result<String, String> {
    if db::update_profile(email, data).await {
        Ok("Profile updated successfully. Please login again to refresh session!".to_owned())
    } else {
        Err("Failed to update profile".to_owned())
    }
}

#[tauri::command]
async fn change_password(email: &str, current: &str, new: &str) -> Result<String, String> {
    if is_credentials_valid(email, current).await {
        if db::change_password(email, new).await {
            Ok("Password changed successfully. Please login again to refresh session!".to_owned())
        } else {
            Err("Failed to change password".to_owned())
        }
    } else {
        Err("The current password provided is incorrect.".to_owned())
    }
}

#[tauri::command]
async fn get_feedbacks(
    class: &str,
    start: Option<&str>,
    end: Option<&str>,
) -> Result<String, String> {
    let feedback_type = FeedbackType::parse(class);
    match feedback_type {
        Ok(feedback_type) => match (end, start) {
            (Some(end), Some(start)) => {
                let filter = DateRangeFilter { start, end };
                let feedbacks = db::get_filtered_feedbacks(feedback_type, filter).await;
                match feedbacks {
                    Some(feedbacks) => Ok(serde_json::to_string(&feedbacks).unwrap()),
                    None => Err("No feedbacks found!".to_owned()),
                }
            }
            _ => {
                let feedbacks = db::get_feedbacks(feedback_type).await;
                match feedbacks {
                    Some(feedbacks) => Ok(serde_json::to_string(&feedbacks).unwrap()),
                    None => Err("No feedbacks found!".to_owned()),
                }
            }
        },
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn start_archive() -> Result<String, String> {
    let archive_count = archive_feedbacks().await;
    if archive_count > 0 {
        Ok(format!("Archived {} records successfully!", archive_count))
    } else {
        Err("No data to archive.".to_owned())
    }
}

#[tauri::command]
async fn generate_report(start: &str, end: &str) -> Result<String, String> {
    let configs = db::get_configs().await;
    if configs.is_none() {
        return Err("Failed to load configs!".to_owned());
    }
    let configs = configs.unwrap();
    let email_recipient = &configs
        .iter()
        .find(|x| x.name == "email_recipient")
        .unwrap_or(&ConfigData {
            name: "email_recipient".to_owned(),
            value: "".to_owned(),
        })
        .value
        .clone();
    if email_recipient.is_empty() {
        notifications::warn("Email Recipient is not configured. Report will not be sent");
        return Err("Email Recipient is not configured. Report will not be sent".to_owned());
    }

    let filter = DateRangeFilter { start, end };
    let feedback_data = db::get_filtered_feedbacks(FeedbackType::Hybrid, filter).await;
    if feedback_data.is_none() {
        return Err("No feedback data available.".to_owned());
    }
    let report = reports::generate_pdf(feedback_data.unwrap()).await;
    match report {
        Some(report) => {
            if mailer::send_report(email_recipient, &report) {
                Ok("Report has been emailed to the supervisor successfully!".to_owned())
            } else {
                Err("Failed to email report.".to_owned())
            }
        }
        None => Err("Failed to generate report!".to_owned()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            authenticate,
            get_session,
            start_face_recording,
            submit_feedback,
            clear_recording,
            get_configs,
            save_configs,
            take_photo,
            update_profile,
            change_password,
            get_feedbacks,
            generate_report,
            get_services,
            add_service,
            delete_service,
            edit_service,
            start_archive
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
