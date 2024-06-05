// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

pub mod ngl;
pub mod utils;

#[derive(Serialize)]
struct SpamResult {
    message: String,
    success: u8,
    failed: u8,
}

#[tauri::command]
async fn spam(
    target: &str,
    count: u8,
    random: bool,
    message: Option<&str>,
) -> Result<SpamResult, ()> {
    let mut success_count = 0;
    let mut fail_count = 0;

    if !random && message.is_none() {
        return Ok(SpamResult {
            message: "Random Message is off but you did not provide a message to send!".to_owned(),
            success: 0,
            failed: 0,
        });
    }

    for _x in 0..count {
        let msg = if random {
            utils::get_random_message()
        } else {
            // NOTE: message can't be none here since we have a check above
            message.unwrap().to_owned()
        };
        let success = ngl::send(target, &msg).await;
        if success {
            success_count += 1;
            continue;
        }
        fail_count += 1;
    }
    println!("SPAM COMPLETE\nSuccess: {success_count}\nFailure: {fail_count}");
    Ok(SpamResult {
        message: "Spam Complete!".to_owned(),
        success: success_count,
        failed: fail_count,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![spam])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
