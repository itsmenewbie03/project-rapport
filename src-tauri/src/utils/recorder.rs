use std::collections::HashMap;

pub async fn start(feedback_id: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/start_recording";
    let mut params = HashMap::new();
    params.insert("feedback_id", feedback_id);
    let request = client.post(url).form(&params);
    let response = request.send().await.unwrap();
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await.unwrap().to_owned()),
        Err(_) => Err(response.text().await.unwrap()),
    }
}

pub async fn clear(feedback_id: &str) {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/clear_recording";
    let mut params = HashMap::new();
    params.insert("feedback_id", feedback_id);
    let request = client.post(url).form(&params);
    let response = request.send().await.unwrap().text().await.unwrap();
    println!("[RUST]: {}", response);
}

pub async fn stop() {
    let client = reqwest::Client::new();
    let url = "http://localhost:5000/stop_recording";
    let request = client.get(url);
    let response = request.send().await.unwrap().text().await.unwrap();
    println!("[RUST]: {}", response);
}
