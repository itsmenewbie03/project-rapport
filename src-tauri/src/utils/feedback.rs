use std::{fs::File, io::Write};
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

impl FeedbackData {
    pub fn parse(data: &str) -> Result<Self, String> {
        match serde_json::from_str(data) {
            Ok(feedback_data) => Ok(feedback_data),
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn save(&self, id: &str) {
        let mut file =
            File::create(format!("{}_feedback.json", id)).expect("Failed to create a file!");
        let feedback_data =
            serde_json::to_string(self).expect("Failed to convert feedback data to json");
        file.write_all(feedback_data.as_bytes())
            .expect("Failed to write feedback data to file");
    }
}
