use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Report", rename_all = "snake-case")]
pub struct Report {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,

    #[structopt(help = "The id of the incident (integer)")]
    incident_id: u64,

    #[structopt(help = "Add additional effort daily, (hours)", short)]
    additional_effort: u64,

    #[structopt(help = "Set up remaining effort, (hours)", short)]
    remaining_effort: u64,
}

impl Report {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let maybe_incident = client.incident.get(self.project_id, self.incident_id).await;

        if maybe_incident.is_ok() {
            let mut incident = maybe_incident?;
            let link = incident.get_link();

            incident.actual_effort =
                Some(incident.actual_effort.unwrap_or(0) + self.additional_effort * 60);

            if self.remaining_effort == 0 {
                incident.remaining_effort = Some(0);
            } else {
                incident.remaining_effort = Some(self.remaining_effort * 60);
            }

            client.incident.update(self.project_id, incident).await?;
            println!("{} - Time updated", link);
        } else {
            panic!(
                "Could not find the requested incident {} in project {}",
                self.incident_id, self.project_id
            );
        }

        Ok(())
    }
}
