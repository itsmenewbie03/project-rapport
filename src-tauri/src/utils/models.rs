use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct ConfigData {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct FeedbackDataRow {
    pub id: i32,
    pub data: String,
    pub created_at: String,
}
