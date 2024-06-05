use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::utils;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub question: String,
    pub deviceId: String,
    pub gameSlug: Option<String>,
    pub referrer: String,
}

pub async fn send(target: &str, message: &str) -> bool {
    let uuid = utils::get_random_uuid();
    println!(
        "Target:  {} | Message: {} | UUID: {}",
        target, message, uuid
    );
    let url = "https://ngl.link/api/submit";
    let cli = Client::new();
    let body = RequestBody {
        username: target.to_owned(),
        question: message.to_owned(),
        deviceId: uuid,
        gameSlug: None,
        referrer: "https://l.facebook.com/".to_owned(),
    };
    let res = cli.post(url).form(&body).send().await.unwrap();
    res.status().is_success()
}
