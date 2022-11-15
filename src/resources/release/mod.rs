use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use self::list::List;

pub mod list;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", about = "Release Cli", rename_all = "kebab-case")]
pub enum ReleaseCli {
    List(List),
}

impl ReleaseCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            ReleaseCli::List(list) => list.run(client).await?,
        }

        Ok(())
    }
}
