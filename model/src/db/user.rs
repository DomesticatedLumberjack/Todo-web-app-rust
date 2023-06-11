use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct User{
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub create_time: String
}