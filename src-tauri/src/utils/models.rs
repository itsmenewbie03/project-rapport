use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct ConfigData {
    pub name: String,
    pub value: String,
}
