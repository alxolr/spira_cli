use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Delete Task", rename_all = "kebab-case")]
pub struct Delete {
    project_id: u64,
    task_id: u64,
}

impl Delete {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        client.task.delete(self.project_id, self.task_id).await?;
        println!("Task {} was succesfully removed", self.task_id);

        Ok(())
    }
}
