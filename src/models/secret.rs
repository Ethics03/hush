use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Secret {
    pub key: String,
    pub value: String,
    pub created_at: i64,
}
