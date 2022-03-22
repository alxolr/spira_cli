use spira::{resources::incident::IncidentDto, SpiraClient};
use std::{error::Error, io::BufReader, path::PathBuf};
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "Create Incidents", rename_all = "kebab-case")]
pub struct Create {
    #[structopt(
        help = "Yaml file path to load the incidents from.",
        default_value = "./incidents.yaml"
    )]
    pub path: PathBuf,
}

impl Create {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let input = std::fs::File::open(&self.path).expect("Provided file does not exist.");
        let rdr = BufReader::new(input);
        let incidents: Vec<IncidentDto> =
            serde_yaml::from_reader(rdr).expect("Could not parse the yaml file.");

        for incident in incidents {
            let incident = client
                .incident
                .create(incident.project_id, incident)
                .await?;

            println!("{}\n{}\n", incident.name, incident.get_link());
        }

        Ok(())
    }
}
