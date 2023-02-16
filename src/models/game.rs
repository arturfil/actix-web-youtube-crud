use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub day: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub day: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub field_name: Option<String>,
    pub address: Option<String>,
    pub day: Option<String>
}

