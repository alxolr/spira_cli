use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Complete Task", rename_all = "kebab-case")]
pub struct Complete {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the task (integer)")]
    task_id: u64,
}

impl Complete {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let maybe_task = client.task.get(self.project_id, self.task_id).await;

        if maybe_task.is_ok() {
            let mut task = maybe_task?;
            let link = task.get_link();
            task.actual_effort = task.estimated_effort;
            task.remaining_effort = Some(0u64);
            task.task_status_id = Some(3); // Mark the status as completed

            client.task.update(self.project_id, task).await?;
            println!("{} - Completed", link);
        } else {
            panic!(
                "Could not find the requested task {} in project {}",
                self.task_id, self.project_id
            );
        }

        Ok(())
    }
}
