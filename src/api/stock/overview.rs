// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332

use std::fmt;
use std::fmt::Debug;

use crate::models::*;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StockOverviewResponse {
    current_stock: Vec<Stock>,
    current_stock_locations: Vec<Stock>,
}

#[derive(Debug)]
struct DieselError(diesel::result::Error);

impl fmt::Display for DieselError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl actix_web::error::ResponseError for DieselError {}

fn action_stock_overview<T: Connection + 'static>(
    connection: PooledConnection<ConnectionManager<T>>,
) -> QueryResult<StockOverviewResponse> {
    use crate::schema::stock::dsl::*;
    Ok(StockOverviewResponse {
        current_stock: stock.load::<Stock>(&connection)?,
        current_stock_locations: stock.load::<Stock>(&connection)?,
    })
}

#[derive(Debug)]
struct R2D2Error(r2d2::Error);

impl fmt::Display for R2D2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl actix_web::error::ResponseError for R2D2Error {}

// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332
pub async fn index<T: Connection + 'static>(pool: web::Data<r2d2::Pool<ConnectionManager<T>>>) -> actix_web::Result<HttpResponse> {
    let connection = pool.get().map_err(R2D2Error)?;
    Ok(HttpResponse::Ok()
        .json(web::block(move || action_stock_overview(connection).map_err(DieselError)).await?))
}
