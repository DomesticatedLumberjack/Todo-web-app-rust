use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskDeleteRequest{
    pub user_id: String,
    pub task_id: String
} 