use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskCreateRequest{
    pub user_id: String,
    pub description: String,
    pub complete: bool
} 