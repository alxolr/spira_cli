pub mod change;
pub mod create;
pub mod delete;
pub mod get;
pub mod link;
pub mod update;

use std::env;

use spira::{resources::incident::IncidentDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

use self::{change::Change, create::Create, delete::Delete, get::Get, link::Link, update::Update};

use super::UiLink;

impl UiLink for IncidentDto {
    fn get_link(&self) -> String {
        let path = env::var("SPIRA_BASE_URL").expect("Envar SPIRA_BASE_URL is not set");
        format!(
            "{}/{}/Incident/{}.aspx",
            path,
            self.project_id,
            self.incident_id.unwrap()
        )
    }
}

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", about = "Incident Cli", rename_all = "kebab-case")]
pub enum IncidentCli {
    Create(Create),
    Delete(Delete),
    Get(Get),
    Update(Update),
    Change(Change),
    Link(Link),
}

impl IncidentCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            IncidentCli::Create(task) => task.run(client).await?,
            IncidentCli::Delete(task) => task.run(client).await?,
            IncidentCli::Get(task) => task.run(client).await?,
            IncidentCli::Update(task) => task.run(client).await?,
            IncidentCli::Change(task) => task.run(client).await?,
            IncidentCli::Link(task) => task.run().await?,
        }

        Ok(())
    }
}
