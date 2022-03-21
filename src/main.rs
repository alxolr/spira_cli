use dotenv::dotenv;
use resources::task::TaskCli;
use spira::SpiraClient;
use std::env;
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
    let spira_cli = Spira::from_args();

    match spira_cli {
        Spira::Task(task) => task.run(&spira_client).await?,
    }

    Ok(())
}
