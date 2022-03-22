use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete Incident", rename_all = "kebab-case")]
pub struct Delete {
    project_id: u64,
    incident_id: u64,
}

impl Delete {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        client
            .incident
            .delete(self.project_id, self.incident_id)
            .await?;
        println!("Incident {} was succesfully removed", self.incident_id);

        Ok(())
    }
}
