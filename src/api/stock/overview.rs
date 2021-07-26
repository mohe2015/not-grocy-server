// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332

use std::fmt::Debug;
use std::str;

use crate::api::utils::DieselError;
use crate::api::utils::R2D2Error;
use crate::models::*;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::connection::AnsiTransactionManager;
use diesel::prelude::*;
use diesel::query_dsl::InternalJoinDsl;
use diesel::query_dsl::JoinWithImplicitOnClause;
use diesel::r2d2::ConnectionManager;
use diesel::types::FromSql;
use diesel::types::HasSqlType;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StockOverviewResponse {
    //current_stock: Vec<Stock>,
//current_stock_locations: Vec<(Stock, Location)>,
}

// https://stackoverflow.com/questions/62746540/diesel-with-custom-wrapper-types
fn action<T>(
    connection: PooledConnection<ConnectionManager<T>>,
) -> QueryResult<StockOverviewResponse>
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
    use crate::schema::products::dsl::*;
    use crate::schema::quantity_units::dsl::*;
    use crate::schema::stock::dsl::*;
    let teee = products
        .inner_join(quantity_units.on(qu_id_purchase.eq(crate::schema::quantity_units::dsl::id)))
        .load::<(Product, QuantityUnit)>(&connection);
    //let the_stock = stock.load::<Stock>(&connection)?;
    //let the_products = products.load::<Product>(&connection)?;
    //the_stock.iter().zip()
    //let data = stocks.into_iter().zip(the_products).collect::<Vec<_>>();
    Ok(StockOverviewResponse {
        //current_stock: stocks,
        //current_stock_locations: stock.inner_join(locations).load::<(Stock, Location)>(&connection)?,
    })
}

// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332
pub async fn index<T>(
    pool: web::Data<r2d2::Pool<ConnectionManager<T>>>,
) -> actix_web::Result<HttpResponse>
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
    let connection = pool.get().map_err(R2D2Error)?;
    let json = web::block(move || action(connection).map_err(DieselError)).await??;
    Ok(HttpResponse::Ok().json(json))
}
