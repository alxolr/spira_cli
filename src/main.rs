use dotenv::dotenv;
use spira_client::{entities::task::TaskDto, SpiraClient};
use std::{env, io::BufReader, path::Path};

mod spira_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load envars
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_URL")?;

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    let tasks = get_yaml_tasks();

    let users = spira_client.user.list(33).await?;


    // let task = spira_client.task.create(33, task).await?;
    // let task = spira_client.task.get(33, 31005).await?;

    println!("{:#?}", users);

    Ok(())
}

fn get_yaml_tasks() -> Vec<TaskDto> {
    let arg = env::args().skip(1).last().unwrap();
    let path = Path::new(&arg);
    let input = std::fs::File::open(path).expect("Provided file does not exist");
    let rdr = BufReader::new(input);
    let tasks: Vec<TaskDto> =
        serde_yaml::from_reader(rdr).expect("Could not process the tasks.yaml file");

    tasks
}
