use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct ConfigData {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct ServiceData {
    pub name: String,
}

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct FeedbackDataRow {
    pub id: i32,
    pub data: String,
    pub tag: String,
    pub created_at: u32,
}

// NOTE: i wrote some lifetimes for the first time
// because I think I undestand what I'm doing
// do I tho?
pub struct DateRangeFilter<'a> {
    pub start: &'a str,
    pub end: &'a str,
}
