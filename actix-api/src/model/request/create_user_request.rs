use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest{
    pub username: String,
    pub password: String
}
