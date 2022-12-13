use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Change incident status and owner", rename_all = "kebab-case")]
pub struct Change {
    project_id: u64,
    incident_id: u64,

    #[structopt(help = "New status of the incident", short)]
    status_id: Option<u64>,

    #[structopt(help = "New owner of the incident", short)]
    owner_id: Option<u64>,
}

impl Change {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let mut incident = client
            .incident
            .get(self.project_id, self.incident_id)
            .await?;
        let link = incident.get_link();

        if self.status_id.is_some() {
            incident.incident_status_id = self.status_id;
        }

        if self.owner_id.is_some() {
            incident.owner_id = self.owner_id;
        }

        client.incident.update(self.project_id, incident).await?;

        println!("{} - was updated", link);

        Ok(())
    }
}
