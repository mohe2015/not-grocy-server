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
use actix_web::web::Data;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use actix_web::{App, HttpServer};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::connection::AnsiTransactionManager;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use diesel::types::{FromSql, HasSqlType};
use diesel::Connection;
use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::Pool;

// added endpoints in fork:
/*
$group->get('/system/config/grocy', '\Grocy\Controllers\SystemApiController:GetGrocyConfig');
$group->get('/system/config/units', '\Grocy\Controllers\SystemApiController:GetQuantitiyUnitConfig');
$group->get('/stock/products', '\Grocy\Controllers\StockApiController:GetProducts');
$group->get('/stock/overview', '\Grocy\Controllers\StockApiController:StockOverview');
$group->get('/recipes', '\Grocy\Controllers\RecipesApiController:GetAll');
$group->get('/recipes/{recipeId}/get', '\Grocy\Controllers\RecipesApiController:GetRecipe');
$group->put('/recipes/{recipeId}/edit', '\Grocy\Controllers\RecipesApiController:EditRecipe');
*/
async fn handler<'a>(real_request: HttpRequest) -> actix_web::Result<HttpResponse> {
    // https://github.com/actix/actix-web/pull/2113
    // https://github.com/actix/actix-web/issues?q=is%3Aissue+is%3Aopen+client
    // https://github.com/actix/actix-web/issues/2287
    // https://github.com/actix/actix-web/issues/2109
    // https://www.reddit.com/r/learnrust/comments/9hhr0u/actixwebclient_how_to_get_body_of_response/
    // the following code is ugly but awc is buggy.
    let request_url = format!("http://localhost:8000{}", real_request.path());
    let response = reqwest::get(&request_url)
        .await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let status = response.status();
    let mut r = &mut HttpResponse::build(status);
    let headers = response.headers();
    for (k, v) in headers {
        r = r.insert_header((k.as_str(), v.as_bytes()));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(r.body(bytes.to_vec()))
}

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
    f32: FromSql<diesel::sql_types::Float, <T as diesel::Connection>::Backend>,
    *const str: FromSql<diesel::sql_types::Text, <T as diesel::Connection>::Backend>,
{
    let pool: Pool<ConnectionManager<T>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        // TODO FIXME REMOVE
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .send_wildcard();

        App::new()
            .wrap(cors)
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
            .default_service(web::get().to(handler))
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
