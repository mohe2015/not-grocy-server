use actix_web::web;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::connection::AnsiTransactionManager;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use diesel::types::{FromSql, HasSqlType};
use diesel::Connection;
use diesel::PgConnection;
use dotenv::dotenv;
use not_grocy_server::api;
use r2d2::Pool;
use std::env;

use actix_web::{App, HttpServer};

// https://stackoverflow.com/questions/65645622/how-do-i-pass-a-trait-as-application-data-to-actix-web
async fn run<T>(manager: ConnectionManager<T>) -> std::io::Result<()>
where
    T: Connection<TransactionManager = AnsiTransactionManager> + 'static,
    <T>::Backend: UsesAnsiSavepointSyntax,
    <T>::Backend: HasSqlType<diesel::sql_types::Bool>,
    bool: FromSql<diesel::sql_types::Bool, <T>::Backend>,
    NaiveDate: FromSql<diesel::sql_types::Date, <T>::Backend>,
    NaiveDateTime: FromSql<diesel::sql_types::Timestamp, <T>::Backend>,
    i32: FromSql<diesel::sql_types::Integer, <T as diesel::Connection>::Backend>,
    f64: FromSql<diesel::sql_types::Double, <T as diesel::Connection>::Backend>,
    *const str: FromSql<diesel::sql_types::Text, <T as diesel::Connection>::Backend>,
{
    let pool: Pool<ConnectionManager<T>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        App::new().data(pool.clone()).route(
            "/api/stock/overview",
            web::get().to(api::stock::overview::index::<T>),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if database_url.starts_with("postgres://") {
        run(ConnectionManager::<PgConnection>::new(database_url)).await
    } else {
        run(ConnectionManager::<SqliteConnection>::new(database_url)).await
    }
}