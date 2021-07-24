// This file contains parts of https://github.com/grocy/grocy Copyright (c) 2017 Bernd Bestel which is licensed under the MIT License.
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_id: i32,
    pub amount: f64,
    pub best_before_date: Option<NaiveDate>,
    pub purchased_date: Option<NaiveDate>,
    pub stock_id: String,
    pub price: Option<f64>,
    pub open: bool,
    pub opened_date: Option<NaiveDateTime>,
    pub row_created_timestamp: Option<NaiveDateTime>,
    pub location_id: Option<i32>,
    pub shopping_location_id: Option<i32>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    product_group_id: Option<i32>,
    active: bool,
    location_id: i32,
    shopping_location_id: Option<i32>,
    qu_id_purchase: i32,
    qu_id_stock: i32,
    qu_factor_purchase_to_stock: f32,
    min_stock_amount: i32,
    default_best_before_days: i32,
    default_best_before_days_after_open: i32,
    default_best_before_days_after_freezing: i32,
    default_best_before_days_after_thawing: i32,
    picture_file_name: Option<String>,
    enable_tare_weight_handling: bool,
    tare_weight: f32,
    not_check_stock_fulfillment_for_recipes: Option<bool>,
    parent_product_id: Option<i32>,
    calories: Option<i32>,
    cumulate_min_stock_amount_of_sub_products: Option<bool>,
    due_type: bool,
    quick_consume_amount: f32,
    hide_on_stock_overview: bool,
    row_created_timestamp: Option<NaiveDateTime>,
    default_print_stock_label: i32,
    allow_label_per_unit: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct QuantityUnit {
    id: i32,
    name: String,
    description: Option<String>,
    row_created_timestamp: Option<NaiveDateTime>,
    name_plural: Option<String>,
    plural_forms: Option<String>,
}
