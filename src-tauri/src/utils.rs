use rand::seq::SliceRandom;
use rand::thread_rng;

const QUESTIONS: &str = include_str!("./questions.json");

fn load_questions() -> Vec<String> {
    serde_json::from_str(QUESTIONS).unwrap()
}

pub fn get_random_message() -> String {
    let questions: Vec<String> = load_questions();
    let mut rng = thread_rng();
    // NOTE: we can afford clones here it's just a single string xD
    questions
        .choose(&mut rng)
        .unwrap_or(&"Why are you gae?".to_owned())
        .to_owned()
}

pub fn get_random_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}
