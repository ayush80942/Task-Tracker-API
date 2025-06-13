use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct TaskInput {
    pub title: String,
}
