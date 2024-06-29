use crate::utils::{db, mailer, models::ConfigData, notifications};

pub async fn check_threshold(feedback_category: &str) {
    // TODO: since the threshold would be a simple resettable counter
    // let's take advantage of ENV vars
    let mut current_count = std::env::var("NEGATIVE_FEEDBACK_HITS")
        .unwrap_or_else(|_| {
            // TODO: set the value
            std::env::set_var("NEGATIVE_FEEDBACK_HITS", "0");
            "0".to_owned()
        })
        .parse::<u8>()
        .unwrap_or(0);
    if feedback_category == "negative" {
        current_count += 1;
        std::env::set_var("NEGATIVE_FEEDBACK_HITS", current_count.to_string());
    }
    let configs = db::get_configs().await;
    match configs {
        Some(configs) => {
            let threshold = configs
                .iter()
                .find(|x| x.name == "max_negative_feedbacks")
                .unwrap_or(&ConfigData {
                    name: "max_negative_feedbacks".to_owned(),
                    value: "3".to_owned(),
                })
                .value
                .parse::<u8>()
                .unwrap_or(3);
            if current_count >= threshold {
                println!("[RUST]: limit reached {}, send mail now!", threshold);
                let email_recipient = &configs
                    .iter()
                    .find(|x| x.name == "email_recipient")
                    .unwrap_or(&ConfigData {
                        name: "email_recipient".to_owned(),
                        value: "".to_owned(),
                    })
                    .value
                    .clone();
                if email_recipient.is_empty() {
                    notifications::warn("Email Recipient is not configured. Mail will not be sent");
                    return;
                };
                mailer::send(email_recipient);
                // TODO: reset the count
                std::env::set_var("NEGATIVE_FEEDBACK_HITS", "0");
            }
        }
        None => notifications::warn("Failed to load configs!"),
    }
}
