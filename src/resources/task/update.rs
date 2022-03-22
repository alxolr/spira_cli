use spira::{resources::task::TaskDto, SpiraClient};
use std::{error::Error, io::BufReader, path::PathBuf};
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Update Tasks", rename_all = "kebab-case")]
pub struct Update {
    #[structopt(
        help = "Yaml file path to load the tasks from.",
        default_value = "./tasks.yaml"
    )]
    pub path: PathBuf,
}

impl Update {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let input = std::fs::File::open(&self.path).expect("Provided file does not exist");
        let rdr = BufReader::new(input);
        let tasks: Vec<TaskDto> =
            serde_yaml::from_reader(rdr).expect("Could not parse the yaml file");

        for task in tasks {
            let link = task.get_link();
            let id = task.task_id.unwrap();
            client.task.update(task.project_id, task).await?;
            
            println!("Task {} was updated\n{}", id, link);
        }

        Ok(())
    }
}
