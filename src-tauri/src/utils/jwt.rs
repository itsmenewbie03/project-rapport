use std::env;

use dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::notifications::warn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: i64,
    pub name: String,
    pub email: String,
}

pub fn generate(claims: Claims) -> Option<String> {
    dotenv::dotenv().ok();
    match env::var("AUTH_SECRET") {
        Ok(secret) => {
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            ) {
                Ok(token) => Some(token),
                Err(err) => {
                    println!("[RUST]: failed to generate token\nERROR: {}", err);
                    None
                }
            }
        }
        Err(err) => {
            warn("The system detected a missing required environment variable. This will cause all authentication to fail.\nERR_CODE: 100");
            println!("[RUST]: failed to read env for the secret\nERROR: {}", err);
            None
        }
    }
}

pub fn validate(token: &str) -> Option<Claims> {
    dotenv::dotenv().ok();
    match env::var("AUTH_SECRET") {
        Ok(secret) => {
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    println!("[RUST]: token validation success");
                    Some(token_data.claims)
                }
                Err(err) => {
                    println!("[RUST]: failed to validate token\nERROR: {}", err);
                    None
                }
            }
        }
        Err(err) => {
            warn("The system detected a missing required environment variable. This will cause all authentication to fail.\nERR_CODE: 100");
            println!("[RUST]: failed to read env for the secret\nERROR: {}", err);
            None
        }
    }
}

pub fn get_exp() -> u64 {
    jsonwebtoken::get_current_timestamp() + 24 * 60 * 60
}
