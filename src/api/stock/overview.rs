// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332

use std::fmt;
use std::fmt::Debug;
use std::str;

use crate::models::*;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::connection::AnsiTransactionManager;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::types::FromSql;
use diesel::types::HasSqlType;
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

fn action_stock_overview<T>(
    connection: PooledConnection<ConnectionManager<T>>,
) -> QueryResult<StockOverviewResponse>
where
    T: Connection<TransactionManager = AnsiTransactionManager> + 'static,
    NaiveDate: FromSql<diesel::sql_types::Date, <T as diesel::Connection>::Backend>,
    <T as diesel::Connection>::Backend: HasSqlType<diesel::sql_types::Bool>,
    <T as diesel::Connection>::Backend: UsesAnsiSavepointSyntax,
    NaiveDateTime: FromSql<diesel::sql_types::Timestamp, <T as diesel::Connection>::Backend>,
    bool: FromSql<diesel::sql_types::Bool, <T as diesel::Connection>::Backend>,
    <T as diesel::Connection>::Backend: HasSqlType<diesel::sql_types::Text>,
{
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
pub async fn index<T>(
    pool: web::Data<r2d2::Pool<ConnectionManager<T>>>,
) -> actix_web::Result<HttpResponse>
where
    T: Connection<TransactionManager = AnsiTransactionManager> + 'static,
    NaiveDate: FromSql<diesel::sql_types::Date, <T as diesel::Connection>::Backend>,
    <T as diesel::Connection>::Backend: HasSqlType<diesel::sql_types::Bool>,
    <T as diesel::Connection>::Backend: UsesAnsiSavepointSyntax,
    NaiveDateTime: FromSql<diesel::sql_types::Timestamp, <T as diesel::Connection>::Backend>,
    bool: FromSql<diesel::sql_types::Bool, <T as diesel::Connection>::Backend>,
{
    let connection = pool.get().map_err(R2D2Error)?;
    Ok(HttpResponse::Ok()
        .json(web::block(move || action_stock_overview(connection).map_err(DieselError)).await?))
}
