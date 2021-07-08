// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332

use crate::models::*;
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/api/stock/overview")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::schema::stock::dsl::*;

    let connection = pool.get().expect("couldn't get db connection from pool");

    Ok(web::block(move || stock.load::<Stock>(&connection))
        .await
        .map(|the_stock| HttpResponse::Ok().json(the_stock))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
