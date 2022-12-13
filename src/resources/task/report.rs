use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::{task::TaskStatus, UiLink};

#[derive(StructOpt, Debug)]
#[structopt(about = "Report", rename_all = "snake-case")]
pub struct Report {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,

    #[structopt(help = "The id of the task (integer)")]
    task_id: u64,

    #[structopt(help = "Add additional effort daily, (hours)", short)]
    additional_effort: u64,

    #[structopt(help = "Set up remaining effort, (hours)", short)]
    remaining_effort: u64,
}

impl Report {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let maybe_task = client.task.get(self.project_id, self.task_id).await;

        if maybe_task.is_ok() {
            let mut task = maybe_task?;
            let link = task.get_link();

            task.actual_effort =
                Some(task.actual_effort.unwrap_or(0) + self.additional_effort * 60);

            if self.remaining_effort == 0 {
                task.remaining_effort = Some(0);
                task.task_status_id = Some(TaskStatus::Complete as u64);
            } else {
                task.remaining_effort = Some(self.remaining_effort * 60);
            }

            client.task.update(self.project_id, task).await?;
            println!("{} - Time updated", link);
        } else {
            panic!(
                "Could not find the requested task {} in project {}",
                self.task_id, self.project_id
            );
        }

        Ok(())
    }
}
