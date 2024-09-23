use std::collections::HashMap;

use dotenv::dotenv;

use super::notifications::warn;

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
        Ok(_) => Ok(response.text().await.unwrap()),
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
        Ok(_) => {
            let resp = response.text().await.unwrap();
            // NOTE: we need to forward this to the analysis api
            println!("[RUST]: capture response: {}", resp);
            let analysis_resp = analyze(&resp).await.unwrap();
            println!("[RUST]: analysis response: {}", analysis_resp);
            Ok(analysis_resp)
        }
        Err(_) => Err(response.text().await.unwrap()),
    }
}

async fn analyze(b64img: &str) -> Result<String, String> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let url = "http://localhost:6969/start_analysis";
    let url = std::env::var("ANALYSIS_API_URL").unwrap_or_else(|_| {
        // NOTE: we're using the fallback URL, this will fail in most cases
        warn("ANALYSIS_API_URL not set, using fallback URL\nNote this might not work as expected");
        url.to_owned()
    });
    let debug = std::env::var("DEBUG").unwrap_or("false".to_owned());
    let mut params = HashMap::new();
    params.insert("b64img", b64img);
    params.insert("debug", &debug);
    let request = client.post(url).form(&params);
    let response = request.send().await.unwrap();
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.text().await.unwrap()),
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
        Ok(_) => Ok(response.text().unwrap()),
        Err(_) => Err(response.text().unwrap()),
    }
}
