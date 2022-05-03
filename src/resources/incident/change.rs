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
    status_id: u64,

    #[structopt(help = "New owner of the incident", short)]
    owner_id: u64,
}

impl Change {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let mut incident = client
            .incident
            .get(self.project_id, self.incident_id)
            .await?;
        let link = incident.get_link();

        incident.incident_status_id = Some(self.status_id);
        incident.owner_id = Some(self.owner_id);

        client.incident.update(self.project_id, incident).await?;

        println!("{} - was updated", link);

        Ok(())
    }
}
