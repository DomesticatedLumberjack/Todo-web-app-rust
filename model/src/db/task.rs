use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Task{
    pub id: String,
    pub user_id: String,
    pub description: String,
    pub complete: bool,
    pub create_date: String,
    pub modified_date: String
}