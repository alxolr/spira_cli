pub mod complete;
pub mod create;
pub mod delete;
pub mod get;
pub mod link;
pub mod update;

use std::env;

use spira::{resources::task::TaskDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

pub enum TaskStatus {
    Complete = 3,
}

use self::{
    complete::Complete, create::Create, delete::Delete, get::Get, link::Link, update::Update,
};

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
    Link(Link),
}

impl TaskCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            TaskCli::Create(cmd) => cmd.run(client).await?,
            TaskCli::Delete(cmd) => cmd.run(client).await?,
            TaskCli::Get(cmd) => cmd.run(client).await?,
            TaskCli::Update(cmd) => cmd.run(client).await?,
            TaskCli::Complete(cmd) => cmd.run(client).await?,
            TaskCli::Link(cmd) => cmd.run().await?,
        }

        Ok(())
    }
}
