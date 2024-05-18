use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}
