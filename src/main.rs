use dotenv::dotenv;
use spira_client::SpiraClient;
use std::env;

mod spira_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load envars
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_URL")?;

    SpiraClient::new(&base_url, &api_key, &username)?;

    Ok(())
}
