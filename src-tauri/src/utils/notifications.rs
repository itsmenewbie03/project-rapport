use tauri::api::notification::{Notification, Sound};

pub fn warn(message: &str) {
    let ctx = tauri::generate_context!();
    let notification = Notification::new(&ctx.config().tauri.bundle.identifier)
        .title("Warning")
        .body(message)
        .sound(Sound::Default)
        .show();
    match notification {
        Ok(_) => println!("[RUST]: Notification shown successfully"),
        Err(err) => println!("[RUST]: Failed to show notification\nERROR: {}", err),
    }
}
