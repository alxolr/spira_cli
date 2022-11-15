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
        let releases = client.release.list(self.project_id).await?;

        for release in releases {
            println!("{} - {}", &release.release_id.unwrap(), &release.full_name.unwrap());
        }

        Ok(())
    }
}
