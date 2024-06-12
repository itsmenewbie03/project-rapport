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
}
