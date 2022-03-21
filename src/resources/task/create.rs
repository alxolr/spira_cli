use spira::{resources::task::TaskDto, SpiraClient};
use std::{error::Error, io::BufReader, path::PathBuf};
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Create Task", rename_all = "kebab-case")]
pub struct Create {
    #[structopt(
        help = "Yaml file path to load the tasks from.",
        default_value = "./tasks.yaml"
    )]
    pub path: PathBuf,
}

impl Create {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let input = std::fs::File::open(&self.path).expect("Provided file does not exist");
        let rdr = BufReader::new(input);
        let tasks: Vec<TaskDto> =
            serde_yaml::from_reader(rdr).expect("Could not parse the yaml file");

        for task in tasks {
            let task = client.task.create(task.project_id, task).await?;
            println!("{}\n{}\n", task.name.as_ref().unwrap(), task.get_link());
        }

        Ok(())
    }
}
