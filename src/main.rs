#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod models;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(api::stock::overview::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
