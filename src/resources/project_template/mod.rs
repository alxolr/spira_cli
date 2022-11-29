use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use self::incident_status::IncidentStatus;

pub mod incident_status;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.1.0",
    about = "Project Template Cli",
    rename_all = "kebab-case"
)]
pub enum ProjectTemplateCli {
    IncidentStatus(IncidentStatus),
}

impl ProjectTemplateCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            ProjectTemplateCli::IncidentStatus(cmd) => cmd.run(client).await?,
        }

        Ok(())
    }
}
