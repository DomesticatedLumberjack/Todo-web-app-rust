use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Task{
    pub id: String,
    pub description: String,
    pub complete: bool
}