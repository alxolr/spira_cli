use std::env;

use self::{change::Change, get::Get};
use spira::{resources::requirement::RequirementDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

use super::UiLink;

pub mod change;
pub mod get;

impl UiLink for RequirementDto {
    fn get_link(&self) -> String {
        let path = env::var("SPIRA_BASE_URL").expect("Envar SPIRA_BASE_URL is not set");
        format!(
            "{}/{}/Requirement/{}.aspx",
            path,
            self.project_id,
            self.requirement_id.unwrap()
        )
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.1.0",
    about = "Requirement Cli",
    rename_all = "kebab-case"
)]
pub enum RequirementCli {
    Get(Get),
    Change(Change),
}

impl RequirementCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            RequirementCli::Get(cmd) => cmd.run(client).await?,
            RequirementCli::Change(cmd) => cmd.run(client).await?,
        }

        Ok(())
    }
}
