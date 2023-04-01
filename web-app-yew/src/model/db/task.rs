use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Task{
    pub _id: String,
    pub description: String,
    pub complete: bool
}