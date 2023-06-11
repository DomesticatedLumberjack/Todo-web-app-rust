use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]

pub struct UpdateTaskRequest{
    pub description: String,
    pub complete: bool
}