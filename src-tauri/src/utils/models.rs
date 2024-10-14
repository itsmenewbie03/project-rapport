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
    pub created_at: i64,
}

// NOTE: we will use this to store the ID of Row
// that we will archive
#[derive(Serialize, Clone, FromRow, Debug)]
pub struct ArtifactRow {
    pub id: i32,
}

// NOTE: i wrote some lifetimes for the first time
// because I think I undestand what I'm doing
// do I tho?
pub struct DateRangeFilter<'a> {
    pub start: &'a str,
    pub end: &'a str,
}
