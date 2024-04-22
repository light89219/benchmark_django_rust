use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatchTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
