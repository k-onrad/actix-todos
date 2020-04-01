use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use crate::db;
use crate::models;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status {
        status: "UP".to_string(),
    })
}

pub async fn get_lists(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_lists(&client).await;

    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_todos(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_todos(&client, path.0).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_todo(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_todo(&client, path.0, path.1).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo_list(
    db_pool: web::Data<Pool>,
    json: web::Json<models::TodoListCreate>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo_list(&client, json.title.clone()).await;

    match result {
        Ok(new_list) => HttpResponse::Ok().json(new_list),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
    json: web::Json<models::TodoCreate>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo(&client, path.0, json.title.clone()).await;

    match result {
        Ok(new_todo) => HttpResponse::Ok().json(new_todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_todo(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::check_todo(&client, path.0, path.1).await;

    match result {
        Ok(checked_todo) => HttpResponse::Ok().json(checked_todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
