use crate::utils::{db, encryption};

pub mod models {
    use serde::Serialize;
    use sqlx::FromRow;
    #[derive(Clone, FromRow, Debug)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub email: String,
        pub password: String,
    }

    #[derive(Serialize)]
    pub struct UserData {
        pub id: i64,
        pub name: String,
        pub email: String,
    }
}

pub async fn is_credentials_valid(email: &str, password: &str) -> bool {
    let user = db::get_user(email).await;
    match user {
        Some(user) => encryption::verify(password, &user.password),
        None => false,
    }
}
