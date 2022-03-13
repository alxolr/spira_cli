use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    #[serde(rename = "TaskStatusId")]
    pub task_status_id: u64,
    #[serde(rename = "RequirementId")]
    pub requirement_id: u64,
    #[serde(rename = "ReleaseId")]
    pub release_id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OwnerId")]
    pub owner_id: u64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EstimatedEffort")]
    pub estimated_effort: u64,
    #[serde(rename = "ActualEffort")]
    pub actual_effort: u64,
    #[serde(rename = "RemainingEffort")]
    pub remaining_effort: u64,
    #[serde(rename = "ProjectId")]
    pub project_id: u64,
}
