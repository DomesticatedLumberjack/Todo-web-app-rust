use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct CreateTaskRequest{
    pub description: String,
    pub complete: bool
}