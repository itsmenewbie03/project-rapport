use crate::utils::db;
use std::{collections::HashMap, fmt::Display, fs::File, io::Write};

#[derive(Debug)]
pub enum FeedbackType {
    Trad,
    Hybrid,
}

impl FeedbackType {
    pub fn parse(data: &str) -> Result<Self, String> {
        match data {
            "trad" => Ok(FeedbackType::Trad),
            "hybrid" => Ok(FeedbackType::Hybrid),
            _ => Err("Invalid feedback type".to_string()),
        }
    }
}

impl Display for FeedbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FeedbackType::Trad => write!(f, "trad_feedback_data"),
            FeedbackType::Hybrid => write!(f, "hybrid_feedback_data"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FeedbackData {
    pub responsiveness: u8,
    pub reliability: u8,
    pub access_and_facilities: u8,
    pub communication: u8,
    pub value_for_money: u8,
    pub integrity: u8,
    pub assurance: u8,
    pub outcome: u8,
    pub overall_satisfaction: u8,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HybridFeedbackData {
    pub feedback_data: FeedbackData,
    pub emotion_data: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

impl FeedbackData {
    pub fn parse(data: &str) -> Result<Self, String> {
        match serde_json::from_str(data) {
            Ok(feedback_data) => Ok(feedback_data),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn save_as_json(&self, id: &str) {
        let mut file =
            File::create(format!("{}_feedback.json", id)).expect("Failed to create a file!");
        let feedback_data =
            serde_json::to_string(self).expect("Failed to convert feedback data to json");
        file.write_all(feedback_data.as_bytes())
            .expect("Failed to write feedback data to file");
    }

    pub async fn save_to_db(&self, feedback_type: FeedbackType, tag: &str) -> bool {
        let data = serde_json::to_string(self).unwrap();
        match feedback_type {
            FeedbackType::Trad => db::save_trad_feedback(&data, tag).await,
            // WARN: we are heavily redesigning the system
            // the follow line will not get called
            // if it get's called then we will tag it as UNTAGGED
            FeedbackType::Hybrid => db::save_hybrid_feedback(&data, "UNTAGGED").await,
        }
    }

    pub fn mean(&self) -> f64 {
        let total = self.responsiveness
            + self.reliability
            + self.access_and_facilities
            + self.communication
            + self.value_for_money
            + self.integrity
            + self.assurance
            + self.outcome
            + self.overall_satisfaction;
        total as f64 / 9.0
    }
}
