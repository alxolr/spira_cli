pub mod complete;
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use std::env;

use spira::{resources::task::TaskDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

use self::{complete::Complete, create::Create, delete::Delete, get::Get, update::Update};

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
#[structopt(about = "Tasks Cli", rename_all = "kebab-case")]
pub enum TaskCli {
    Create(Create),
    Delete(Delete),
    Get(Get),
    Update(Update),
    Complete(Complete),
}

impl TaskCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            TaskCli::Create(create) => create.run(client).await?,
            TaskCli::Delete(delete) => delete.run(client).await?,
            TaskCli::Get(get) => get.run(client).await?,
            TaskCli::Update(update) => update.run(client).await?,
            TaskCli::Complete(complete) => complete.run(client).await?,
        }

        Ok(())
    }
}
