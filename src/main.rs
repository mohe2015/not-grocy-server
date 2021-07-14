#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod models;
pub mod schema;

use actix_web::web;
use diesel::Connection;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

use actix_web::{App, HttpServer};

// https://stackoverflow.com/questions/65645622/how-do-i-pass-a-trait-as-application-data-to-actix-web
async fn run<T: Connection + 'static>(manager: ConnectionManager<T>) -> std::io::Result<()> {
    let pool: Pool<ConnectionManager<T>> = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/api/stock/overview", web::get().to(api::stock::overview::index::<T>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_type = 1;

    if database_type == 1 {
        run(ConnectionManager::<SqliteConnection>::new(database_url)).await
    } else {
        run(ConnectionManager::<PgConnection>::new(database_url)).await
    }
}
