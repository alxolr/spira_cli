use spira::resources::incident::IncidentDto;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Get the incident link", rename_all = "kebab-case")]
pub struct Link {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the incident (integer)")]
    incident_id: u64,
}

impl Link {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let incident = IncidentDto {
            incident_id: Some(self.incident_id),
            project_id: self.project_id,
            ..Default::default()
        };

        println!("{}", incident.get_link());

        Ok(())
    }
}
