use actix_rt;
use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod config;
mod db;
mod models;
mod handlers;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap(); 

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(handlers::status))
            .route("/todos{_:/?}", web::get().to(handlers::get_lists))
            .route("/todos{_:/?}", web::post().to(handlers::create_todo_list))
            .route("/todos/{list_id}{_:/?}", web::get().to(handlers::get_todos))
            .route("/todos/{list_id}{_:/?}", web::post().to(handlers::create_todo))
            .route("/todos/{list_id}/{item_id}{_:/?}", web::get().to(handlers::get_todo))
            .route("/todos/{list_id}/{item_id}{_:/?}", web::put().to(handlers::check_todo))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
