use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub name: String, //changed to String was Option<String> before gotta check if its fine

    pub created_at: i64,

    pub updated_at: i64,

    pub salt: String,

    pub nonce: Option<String>,
}
