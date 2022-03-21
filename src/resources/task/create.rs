use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Create Task", rename_all = "kebab-case")]
pub struct Create {}

impl Create {
    pub async fn run<'a>(&self, client: &'a SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        println!("kind of working");

        Ok(())
    }
}
