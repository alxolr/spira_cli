use dotenv::dotenv;
use resources::UiLink;
use spira::{resources::task::TaskDto, SpiraClient};
use std::{env, io::BufReader, path::Path};

pub mod resources;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load envars
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_API_URL")?;

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;

    let tasks = spira_client.task.list_my().await?;
    for task in tasks {
        println!(
            "{}\n{}\n",
            task.name.as_ref().unwrap(),
            task.get_link()
        );
    }

    // let tasks = get_yaml_tasks();

    // for task in tasks {
    //     let task = spira_client.task.create(task.project_id, task).await?;
    //     println!("{}\n{}\n", task.name.as_ref().unwrap(), task.get_link());
    // }

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
