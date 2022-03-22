use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use self::list::List;

pub mod list;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", about = "Users Cli", rename_all = "kebab-case")]
pub enum UserCli {
    List(List),
}

impl UserCli {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        match self {
            UserCli::List(list) => list.run(client).await?,
        }

        Ok(())
    }
}
