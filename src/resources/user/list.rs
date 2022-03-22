use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "List users", rename_all = "kebab-case")]
pub struct List {
    #[structopt(help = "The id of the project (integer)")]
    project_id: u64,
}

impl List {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let users = client.user.list(self.project_id).await?;

        for user in users {
            println!("{} - {}", &user.user_id.unwrap(), &user.full_name.unwrap());
        }

        Ok(())
    }
}
