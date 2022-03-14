pub mod entities;

use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::time::Duration;

use self::entities::{project::ProjectClient, task::TaskClient, user::UserClient};

pub struct SpiraClient<'a> {
    pub task: TaskClient<'a>,
    pub project: ProjectClient<'a>,
    pub user: UserClient<'a>,
}

type Response<T> = Result<T, Box<dyn std::error::Error>>;

impl<'a> SpiraClient<'a> {
    pub fn new(base_url: &'a str, api_key: &str, username: &str) -> Response<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_str("application/json")?);
        headers.insert("Content-type", HeaderValue::from_str("application/json")?);
        headers.insert("api-key", HeaderValue::from_str(api_key)?);
        headers.insert("username", HeaderValue::from_str(username)?);

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build()?;

        let task = TaskClient::new(client.clone(), base_url);
        let project = ProjectClient::new(client.clone(), base_url);
        let user = UserClient::new(client, base_url);

        Ok(SpiraClient {
            task,
            project,
            user,
        })
    }
}
