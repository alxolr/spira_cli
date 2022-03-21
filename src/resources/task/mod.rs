pub mod create;
pub mod delete;

use std::env;

use spira::{resources::task::TaskDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

use self::{create::Create, delete::Delete};

use super::UiLink;

impl UiLink for TaskDto {
    fn get_link(&self) -> String {
        let path = env::var("SPIRA_BASE_URL").expect("Envar SPIRA_BASE_URL is not set");
        format!(
            "{}/{}/Task/{}.aspx",
            path,
            self.project_id,
            self.task_id.unwrap()
        )
    }
}

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", about = "Task Cli", rename_all = "kebab-case")]
pub enum TaskCli {
    Create(Create),
    Delete(Delete),
}

impl TaskCli {
    pub async fn run<'a>(&self, client: &'a SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            TaskCli::Create(create) => create.run(client).await?,
            TaskCli::Delete(delete) => delete.run(client).await?,
        }

        Ok(())
    }
}
