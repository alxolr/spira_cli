use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Get Requirement", rename_all = "kebab-case")]
pub struct Get {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the requirement (integer)")]
    requirement_id: u64,
}

impl Get {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let requirement = client.requirement.get(self.project_id, self.requirement_id).await?;
        let yaml = serde_yaml::to_string(&vec![requirement])?;
        let result = yaml.replace("---", "");

        println!("{}", result);

        Ok(())
    }
}
