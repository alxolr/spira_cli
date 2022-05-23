use spira::resources::requirement::RequirementDto;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Get the requirement link", rename_all = "kebab-case")]
pub struct Link {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the requirement (integer)")]
    requirement_id: u64,
}

impl Link {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let requirement = RequirementDto {
            requirement_id: Some(self.requirement_id),
            project_id: self.project_id,
            ..Default::default()
        };

        println!("{}", requirement.get_link());

        Ok(())
    }
}
