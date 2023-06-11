use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct User{
    pub id: String,
    pub username: String,
    pub token: String
}