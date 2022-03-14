use crate::spira_client::Response;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDto {
    #[serde(rename = "UserId")]
    pub user_id: u64,
    #[serde(rename = "FullName")]
    pub full_name: String,
}

pub struct UserClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> UserClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        UserClient { client, base_url }
    }

    pub async fn list(&self, project_id: u64) -> Response<Vec<UserDto>> {
        let users = self
            .client
            .get(self.append_to_url(&format!("/projects/{}/users", project_id)))
            .send()
            .await?
            .json::<Vec<UserDto>>()
            .await?;

        Ok(users)
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
