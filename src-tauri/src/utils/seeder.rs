#[tokio::test]
async fn seed() {
    // NOTE: we're crazy to why not use test as a seeder xD
    use super::{db, feedback::FeedbackData};
    use crate::HybridFeedbackData;
    use rand::distributions::Distribution;
    use rand::distributions::Uniform;
    use std::collections::HashMap;

    let count = 1000;
    let range = Uniform::from(1704038400..1735660800);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let feedback_data = FeedbackData::random();
        let feedback_mean = feedback_data.mean();
        let feedback_category = if feedback_mean >= 3.41 {
            "positive"
        } else if feedback_mean < 2.61 {
            "negative"
        } else {
            "neutral"
        };
        let metadata = r#"{"contact_number":"09975125099","office_name":"OVPCASSS","purpose_of_visit":"Free Coffin for those who will die due to a capstone project.","name":"Salvador","client_type":"student","feedback_category":"TEST_ONLY"}"#;

        let metadata: HashMap<String, String> = serde_json::from_str(metadata).unwrap();
        let emotion_data = r#"{"responsiveness":"neutral","reliability":"neutral","outcome":"neutral","overall_satisfaction":"neutral","communication":"happy","integrity":"neutral","access_and_facilities":"neutral","value_for_money":"happy","assurance":"neutral"}"#;
        let emotion_data: HashMap<String, String> = serde_json::from_str(emotion_data).unwrap();
        let hybrid_feedback_data = HybridFeedbackData {
            emotion_data,
            feedback_data,
            metadata,
        };
        let created_at = range.sample(&mut rng);
        db::save_hybrid_feedback_test(
            &serde_json::to_string(&hybrid_feedback_data).unwrap(),
            feedback_category,
            &created_at.to_string(),
        )
        .await;
    }
}
