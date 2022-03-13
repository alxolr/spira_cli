pub mod entities;

use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use std::{collections::HashMap, time::Duration};

use self::entities::{project::Project, user::User};

pub struct SpiraClient<'a> {
    base_url: &'a str,
    client: reqwest::Client,
}

type Response<T> = Result<T, Box<dyn std::error::Error>>;

impl<'a> SpiraClient<'a> {
    pub fn new(base_url: &'a str, api_key: &str, username: &str) -> Response<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_str("application/json")?);
        headers.insert("api-key", HeaderValue::from_str(api_key)?);
        headers.insert("username", HeaderValue::from_str(username)?);

        Ok(SpiraClient {
            base_url,
            client: reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .default_headers(headers)
                .build()?,
        })
    }

    pub async fn get_projects(&self) -> Response<Vec<Project>> {
        let projects = self
            .client
            .get(self.append_to_base("/projects"))
            .send()
            .await?
            .json::<Vec<Project>>()
            .await?;

        Ok(projects)
    }

    pub async fn get_users(&self, project_id: u64) -> Response<HashMap<String, User>> {
        let users = self
            .client
            .get(self.append_to_base(&format!("/projects/{}/users", project_id)))
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;

        let iter = users.into_iter().map(|user| (user.full_name.clone(), user));
        let result = HashMap::from_iter(iter);

        Ok(result)
    }

    pub async fn get_statuses(&self, project_id: u64) -> Response<HashMap<String, User>> {
        let users = self
            .client
            .get(self.append_to_base(&format!("/projects/{}/users", project_id)))
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;

        let iter = users.into_iter().map(|user| (user.full_name.clone(), user));
        let result = HashMap::from_iter(iter);

        Ok(result)
    }

    fn append_to_base(&self, s: &str) -> String {
        format!("{}{}", self.base_url, s)
    }
}
