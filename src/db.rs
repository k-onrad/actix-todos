use crate::models::{TodoList, Todo};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_lists(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("SELECT * FROM todo_lists ORDER BY id DESC").await.unwrap();
    let lists = client.query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(lists)
}

pub async fn get_todos(client: &Client, list_id: i32) -> Result<Vec<Todo>, io::Error> {
    let statement = client.prepare("SELECT * FROM todos WHERE list_id = $1 ORDER BY id DESC").await.unwrap();
    let todos = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting todos")
        .iter()
        .map(|row| Todo::from_row_ref(row).unwrap())
        .collect::<Vec<Todo>>();
    Ok(todos)
}

pub async fn get_todo(client: &Client, list_id: i32, item_id: i32) -> Result<Todo, io::Error> {
    let statement = client.prepare("SELECT * FROM todos WHERE list_id = $1 AND id = $2 ORDER BY id DESC").await.unwrap();
    client.query(&statement, &[&list_id, &item_id])
        .await
        .expect("Error getting todo")
        .iter()
        .map(|row| Todo::from_row_ref(row).unwrap())
        .collect::<Vec<Todo>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error getting todo"))
}

pub async fn create_todo_list(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let statement = client.prepare("INSERT INTO todo_lists (title) VALUES ($1) RETURNING id, title").await.unwrap();
    client.query(&statement, &[&title])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo list"))
}

pub async fn create_todo(client: &Client, list_id: i32, title: String) -> Result<Todo, io::Error> {
    let statement = client.prepare("INSERT INTO todos (list_id, title) VALUES ($1, $2) RETURNING id, title, checked, list_id").await.unwrap();
    client.query(&statement, &[&list_id, &title])
        .await
        .expect("Error getting todo")
        .iter()
        .map(|row| Todo::from_row_ref(row).unwrap())
        .collect::<Vec<Todo>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo"))
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<Todo, io::Error> {
    let statement = client
        .prepare("UPDATE todos SET checked = true WHERE list_id = $1 AND id = $2 AND checked = false RETURNING id, title, checked, list_id")
        .await
        .unwrap();
    client.query(&statement, &[&list_id, &item_id])
        .await
        .expect("Error checking todo")
        .iter()
        .map(|row| Todo::from_row_ref(row).unwrap())
        .collect::<Vec<Todo>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error checking todo"))
}
