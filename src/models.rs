// This file contains parts of https://github.com/grocy/grocy Copyright (c) 2017 Bernd Bestel which is licensed under the MIT License.
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_id: i32,
    //pub amount: f64,
    //pub best_before_date: Option<NaiveDate>,
    //pub purchased_date: Option<NaiveDate>,
    //pub stock_id: String,
    //pub price: Option<f64>,
    //pub open: bool,
    //pub row_created_timestamp: Option<NaiveDateTime>,
    //pub location_id: Option<i32>,
    //pub shopping_location_id: Option<i32>,
    //pub opened_date: Option<NaiveDate>,
}
