use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ProjectId")]
    project_id: u64,
}
