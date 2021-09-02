#[macro_use]
extern crate diesel;

#[path = "../api/mod.rs"]
pub mod api;

#[path = "../models.rs"]
pub mod models;

#[path = "../schema.rs"]
pub mod schema;

use std::env;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use chrono::{NaiveDate, NaiveDateTime};
use diesel::connection::AnsiTransactionManager;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use diesel::types::{FromSql, HasSqlType};
use diesel::Connection;
use diesel::PgConnection;
use diesel::{backend::UsesAnsiSavepointSyntax, MysqlConnection};
use dotenv::dotenv;
use r2d2::Pool;

// https://stackoverflow.com/questions/65645622/how-do-i-pass-a-trait-as-application-data-to-actix-web
async fn run<T>(manager: ConnectionManager<T>) -> Result<(), std::io::Error>
where
    T: Connection<TransactionManager = AnsiTransactionManager> + 'static,
    <T>::Backend: UsesAnsiSavepointSyntax,
    <T>::Backend: HasSqlType<diesel::sql_types::Bool>,
    bool: FromSql<diesel::sql_types::Bool, <T>::Backend>,
    NaiveDate: FromSql<diesel::sql_types::Date, <T>::Backend>,
    NaiveDateTime: FromSql<diesel::sql_types::Timestamp, <T>::Backend>,
    i32: FromSql<diesel::sql_types::Integer, <T as diesel::Connection>::Backend>,
    f64: FromSql<diesel::sql_types::Double, <T as diesel::Connection>::Backend>,
    f32: FromSql<diesel::sql_types::Float, <T as diesel::Connection>::Backend>,
    *const str: FromSql<diesel::sql_types::Text, <T as diesel::Connection>::Backend>,
{
    let pool: Pool<ConnectionManager<T>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    //let connection = pool.get().map_err(R2D2Error)?;
    //sql_query("PRAGMA foreign_keys = ON;")
    //    .execute(&connection)
    //    .map_err(DieselError)?;

    HttpServer::new(move || {
        // TODO FIXME REMOVE
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .send_wildcard();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .route(
                "/api/stock/overview",
                web::get().to(api::stock::overview::index::<T>),
            )
            .route(
                "/api/stock/products",
                web::get().to(api::stock::products::index::<T>),
            )
            .route(
                "/api/objects/quantity_units",
                web::get().to(api::objects::quantity_units::index::<T>),
            )
            .route(
                "/api/system/config/grocy",
                web::get().to(api::system::config::grocy::index::<T>),
            )
            .route("/login", web::get().to(api::login::index::<T>))
            .route("/redirect", web::get().to(api::redirect::index::<T>))
        //.default_service(web::get().to(handler))
    })
    .bind("0.0.0.0:8080")? // TODO FIXME don't listen on all interfaces but docker needs this
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    // https://lib.rs/crates/yup-oauth2
    // https://lib.rs/crates/oauth2 use this for now
    // https://docs.rs/oauth2/4.1.0/oauth2/

    println!("init");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if database_url.starts_with("postgres://") {
        run(ConnectionManager::<PgConnection>::new(database_url)).await
    } else if database_url.starts_with("mysql://") {
        run(ConnectionManager::<MysqlConnection>::new(database_url)).await
    } else {
        run(ConnectionManager::<SqliteConnection>::new(database_url)).await
    }
}
