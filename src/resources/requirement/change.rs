use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Change Requirement status and owner",
    rename_all = "kebab-case"
)]
pub struct Change {
    project_id: u64,
    requirement_id: u64,

    #[structopt(help = "New status of the Requirement", short)]
    status_id: u64,

    #[structopt(help = "New owner of the Requirement", short)]
    owner_id: u64,
}

impl Change {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let mut requirement = client
            .requirement
            .get(self.project_id, self.requirement_id)
            .await?;
        let link = requirement.get_link();

        requirement.status_id = Some(self.status_id);
        requirement.owner_id = Some(self.owner_id);

        client
            .requirement
            .update(self.project_id, requirement)
            .await?;

        println!("{} - was updated", link);

        Ok(())
    }
}
