use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: u64,
    #[serde(rename = "FullName")]
    pub full_name: String,
}
