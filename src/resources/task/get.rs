use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Get Task", rename_all = "kebab-case")]
pub struct Get {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the task (integer)")]
    task_id: u64,
}

impl Get {
    pub async fn run<'a>(&self, client: &'a SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let task = client.task.get(self.project_id, self.task_id).await?;
        let yaml = serde_yaml::to_string(&vec![task])?;
        let result = yaml.replace("---", "");

        println!("{}", result);

        Ok(())
    }
}
