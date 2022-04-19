use spira::{resources::task::TaskDto, SpiraClient};
use std::error::Error;
use structopt::StructOpt;

use crate::resources::{task::TaskStatus, UiLink};

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

            complete_task(&mut task);

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

fn complete_task(task: &mut TaskDto) {
    task.remaining_effort = Some(0u64);

    if task.actual_effort < task.estimated_effort {
        task.actual_effort = task.estimated_effort;
    }

    // Mark the status as completed
    task.task_status_id = Some(TaskStatus::Complete as u64);
}

#[cfg(test)]
mod tests {
    use spira::resources::task::TaskDto;

    use super::*;

    #[test]
    fn test_complete_task_no_time_reported() {
        let mut task = TaskDto::default();
        task.estimated_effort = Some(16u64);
        task.actual_effort = Some(0);

        complete_task(&mut task);

        assert_eq!(task.actual_effort, Some(16u64));
        assert_eq!(task.remaining_effort, Some(0));
    }

    #[test]
    fn test_complete_task_some_time_reported() {
        let mut task = TaskDto::default();
        task.estimated_effort = Some(16u64);
        task.actual_effort = Some(8);

        complete_task(&mut task);

        assert_eq!(task.actual_effort, Some(16u64));
        assert_eq!(task.remaining_effort, Some(0));
    }

    #[test]
    fn test_complete_task_time_reported_is_bigger_than_estimated() {
        let mut task = TaskDto::default();
        task.estimated_effort = Some(16u64);
        task.actual_effort = Some(32);

        complete_task(&mut task);

        assert_eq!(task.actual_effort, Some(32u64));
        assert_eq!(task.remaining_effort, Some(0));
    }
}
