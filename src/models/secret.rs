use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Secret {
    pub key: String,
    pub value: String,
    pub created_at: i64,
}

pub type Secrets = HashMap<String, Secret>;
