use serde_json::Number;
use spira::SpiraClient;
use std::error::Error;
use structopt::StructOpt;

use crate::resources::UiLink;

#[derive(StructOpt, Debug)]
#[structopt(about = "List incidents assigned to me", rename_all = "kebab-case")]
pub struct ListMy {}

impl ListMy {
    pub async fn run(&self, client: &SpiraClient<'_>) -> Result<(), Box<dyn Error>> {
        let incidents = client.incident.list_my().await?;

        incidents.iter().for_each(|incident| {
            let custom_properties = incident.custom_properties.clone().unwrap();

            let item = custom_properties.iter().find(|property| {
                let definition = property.get("Definition").unwrap();
                let name = definition.get("Name").unwrap().to_string();

                name.contains("Business Value")
            });

            if item.is_some() {
                let business_value = item.unwrap().get("IntegerValue").unwrap();
                println!(
                    "{} {} {}",
                    incident.get_link(),
                    business_value,
                    incident.name
                )
            } else {
                println!("{} {}", incident.get_link(), incident.name)
            }
        });

        Ok(())
    }
}
