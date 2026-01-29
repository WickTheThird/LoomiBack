use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "entity_status", rename_all = "lowercase")]
pub enum EntityStatus {
    Active,
    Inactive,
    Pending,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampFields {
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
