use serde::{Deserialize, Serialize};
use super::task::Task;

#[derive(Deserialize, Serialize, Clone)]
pub struct User{
    pub _id: String,
    pub username: String,
    pub password_hash: String,
    pub task_items: Vec<Task>
}