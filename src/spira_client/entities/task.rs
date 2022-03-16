use std::env;

use crate::spira_client::Response;
use reqwest::{header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDto {
    #[serde(rename = "TaskId")]
    pub task_id: Option<u64>,

    #[serde(rename = "ProjectId")]
    pub project_id: u64,

    #[serde(rename = "TaskStatusId")]
    pub task_status_id: u64,

    #[serde(rename = "TaskTypeId")]
    pub task_type_id: u64,

    #[serde(rename = "RequirementId")]
    pub requirement_id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "OwnerId")]
    pub owner_id: u64,

    #[serde(rename = "EstimatedEffort")]
    pub estimated_effort: u64,
}

impl TaskDto {
    pub fn get_link(&self) -> String {
        format!(
            "{}/{}/Task/{}.aspx",
            env::var("SPIRA_BASE_URL").unwrap(),
            self.project_id,
            self.task_id.unwrap()
        )
    }
}

pub struct TaskClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> TaskClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        TaskClient { client, base_url }
    }

    pub async fn get(&self, project_id: u64, task_id: u64) -> Response<TaskDto> {
        let path = &format!("/projects/{}/tasks/{}", project_id, task_id);

        let task = self
            .client
            .get(self.append_to_url(path))
            .send()
            .await?
            .json::<TaskDto>()
            .await?;

        Ok(task)
    }

    pub async fn create(&self, project_id: u64, task: TaskDto) -> Response<TaskDto> {
        let json_task = serde_json::to_string(&task)?;
        let task = self
            .client
            .post(self.append_to_url(&format!("/projects/{}/tasks", project_id)))
            .header("content-type", HeaderValue::from_str("application/json")?)
            .body(json_task)
            .send()
            .await?
            .json::<TaskDto>()
            .await?;

        Ok(task)
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
