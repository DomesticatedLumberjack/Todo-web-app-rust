use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskUpdateRequest{
    pub user_id: String,
    pub task_id: String,
    pub description: String,
    pub complete: bool
}
