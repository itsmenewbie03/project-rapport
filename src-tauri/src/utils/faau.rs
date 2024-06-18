use std::collections::HashMap;

pub async fn start(feedback_id: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/start_analysis";
    let mut params = HashMap::new();
    params.insert("feedback_id", feedback_id);
    // TODO: pull the config data if debug is on
    // for now we will pull it from env
    let debug = std::env::var("DEBUG").unwrap_or("false".to_owned());
    println!("[RUST]: faau debug mode is {}", debug);
    params.insert("debug", &debug);
    let request = client.post(url).form(&params);
    let response = request.send().await.unwrap();
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await.unwrap().to_owned()),
        Err(_) => Err(response.text().await.unwrap()),
    }
}

pub async fn capture(feedback_id: &str, quality: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/take_photo";
    let mut params = HashMap::new();
    params.insert("feedback_id", feedback_id);
    params.insert("quality", quality);
    let request = client.post(url).form(&params);
    let response = request.send().await.unwrap();
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await.unwrap().to_owned()),
        Err(_) => Err(response.text().await.unwrap()),
    }
}

pub fn poll(feedback_id: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let url = "http://localhost:5000/poll_analysis";
    let mut params = HashMap::new();
    params.insert("feedback_id", feedback_id);
    // TODO: pull the config data if debug is on
    // for now we will pull it from env
    let debug = std::env::var("DEBUG").unwrap_or("false".to_owned());
    println!("[RUST]: faau debug mode is {}", debug);
    params.insert("debug", &debug);
    let request = client.post(url).form(&params);
    let response = request.send().unwrap();
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().unwrap().to_owned()),
        Err(_) => Err(response.text().unwrap()),
    }
}
