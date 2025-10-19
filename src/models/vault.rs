use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    name: String,

    created_at: i64,

    updated_at: i64,
}
