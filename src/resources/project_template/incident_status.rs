use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "List incidents statuses", rename_all = "kebab-case")]
pub struct IncidentStatus {
    #[structopt(help = "The id of the project template (integer)")]
    project_template_id: u64,
}

impl IncidentStatus {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let incident_statuses = client
            .project_template
            .incident_status_list(self.project_template_id)
            .await?;

        for status in incident_statuses {
            println!(
                "{} - {}",
                &status.incident_status_id.unwrap(),
                &status.name.unwrap()
            );
        }

        Ok(())
    }
}
