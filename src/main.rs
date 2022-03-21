use dotenv::dotenv;
use resources::task::TaskCli;
use spira::{resources::task::TaskDto, SpiraClient};
use std::{env, io::BufReader, path::Path};
use structopt::StructOpt;

pub mod resources;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", about = "Spira Cli", rename_all = "kebab-case")]
enum Spira {
    Task(TaskCli),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load envars
    let api_key = env::var("SPIRA_API_KEY").expect("Envar SPIRA_API_KEY is not set.");
    let username = env::var("SPIRA_USERNAME").expect("Envar SPIRA_USERNAME is not set.");
    let base_url = env::var("SPIRA_API_URL").expect("Envar SPIRA_API_URL is not set.");

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    
    let cmd = Spira::from_args();

    match cmd {
        Spira::Task(task) => task.run(&spira_client).await?,
    }

    // let tasks = get_yaml_tasks();

    // for task in tasks {
    //     let task = spira_client.task.create(task.project_id, task).await?;
    //     println!("{}\n{}\n", task.name.as_ref().unwrap(), task.get_link());
    // }

    // let requirement = spira_client.requirement.get(33, 23940).await?;

    // println!("{:#?}", requirement);

    Ok(())
}

fn get_yaml_tasks() -> Vec<TaskDto> {
    let arg = env::args()
        .skip(1)
        .last()
        .expect("Yaml file path was not provided");
    let path = Path::new(&arg);

    let input = std::fs::File::open(path).expect("Provided file does not exist");
    let rdr = BufReader::new(input);

    serde_yaml::from_reader(rdr).expect("Could not parse the yaml file")
}
