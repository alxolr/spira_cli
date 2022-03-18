use std::env;

use spira::resources::task::{TaskClient, TaskDto};

use super::UiLink;

impl UiLink for TaskDto {
    fn get_link(&self) -> String {
        let path = env::var("SPIRA_BASE_URL").expect("SPIRA_BASE_URL is not set");
        format!(
            "{}/{}/Task/{}.aspx",
            path,
            self.project_id,
            self.task_id.unwrap()
        )
    }
}

