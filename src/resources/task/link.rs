use spira::resources::task::TaskDto;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Get the task link", rename_all = "kebab-case")]
pub struct Link {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
    #[structopt(help = "The id of the task (integer)")]
    task_id: u64,
}

impl Link {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let task = TaskDto {
            task_id: Some(self.task_id),
            project_id: self.project_id,
            ..Default::default()
        };
        println!("{}", task.get_link());

        Ok(())
    }
}
