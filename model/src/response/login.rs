use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login{
    pub id: String,
    pub username: String,
    pub access_token: String,
}