use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_lists")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TodoListCreate {
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todos")]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct TodoCreate {
    pub title: String,
}
