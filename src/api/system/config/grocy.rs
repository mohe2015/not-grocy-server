// https://github.com/mistressofjellyfish/not-grocy/blob/ddc2dad07ec26f854cca78bbdbec92b2213ad235/php/Controllers/StockApiController.php#L332

use std::fmt;
use std::fmt::Debug;
use std::str;

use crate::api::utils::R2D2Error;
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

// TODO FIXME use Rust to generate OpenAPI schema

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GlobalConfig {
    culture: String,
    currency: String,
    calendar_first_day_of_week: String,
    calendar_show_week_numbers: bool,
    meal_plan_first_day_of_week: String,
    locale: String,
    feature_flags: GlobalConfigFeatureFlags,
    user: GlobalUserConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct GlobalConfigFeatureFlags {
    grocy_feature_flag_stock: bool,
    grocy_feature_flag_shoppinglist: bool,
    grocy_feature_flag_recipes: bool,
    grocy_feature_flag_chores: bool,
    grocy_feature_flag_tasks: bool,
    grocy_feature_flag_batteries: bool,
    grocy_feature_flag_equipment: bool,
    grocy_feature_flag_calendar: bool,
    grocy_feature_flag_labelprinter: bool,
    grocy_feature_flag_stock_price_tracking: bool,
    grocy_feature_flag_stock_location_tracking: bool,
    grocy_feature_flag_stock_best_before_date_tracking: bool,
    grocy_feature_flag_stock_product_opened_tracking: bool,
    grocy_feature_flag_stock_product_freezing: bool,
    grocy_feature_flag_stock_best_before_date_field_number_pad: bool,
    grocy_feature_flag_shoppinglist_multiple_lists: bool,
    grocy_feature_flag_chores_assignments: bool,
    grocy_feature_flag_thermal_printer: bool,
    grocy_feature_flag_auto_torch_on_with_camera: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSettings {
    night_mode_enabled: bool,
    auto_night_mode_enabled: bool,
    auto_night_mode_time_range_from: String,
    auto_night_mode_time_range_to: String,
    auto_night_mode_time_range_goes_over_midnight: bool,
    currently_inside_night_mode_range: bool,
    keep_screen_on: bool,
    keep_screen_on_when_fullscreen_card: bool,
    product_presets_location_id: i32,
    product_presets_product_group_id: i32,
    product_presets_qu_id: i32,
    stock_decimal_places_amounts: i32,
    stock_decimal_places_prices: i32,
    stock_due_soon_days: i32,
    stock_default_purchase_amount: i32,
    stock_default_consume_amount: i32,
    stock_default_consume_amount_use_quick_consume_amount: bool,
    scan_mode_consume_enabled: bool,
    scan_mode_purchase_enabled: bool,
    show_icon_on_stock_overview_page_when_product_is_on_shopping_list: bool,
    show_purchased_date_on_purchase: bool,
    show_warning_on_purchase_when_due_date_is_earlier_than_next: bool,
    shopping_list_to_stock_workflow_auto_submit_when_prefilled: bool,
    shopping_list_show_calendar: bool,
    recipe_ingredients_group_by_product_group: bool,
    chores_due_soon_days: i32,
    batteries_due_soon_days: i32,
    tasks_due_soon_days: i32,
    auto_reload_on_db_change: bool,
    show_clock_in_header: bool,
    quagga2_numofworkers: i32,
    quagga2_halfsample: bool,
    quagga2_patchsize: String,
    quagga2_frequency: i32,
    quagga2_debug: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Permissions {
    admin: bool,
    users: bool,
    users_create: bool,
    users_edit: bool,
    users_read: bool,
    users_edit_self: bool,
    stock: bool,
    shoppinglist: bool,
    recipes: bool,
    chores: bool,
    batteries: bool,
    tasks: bool,
    equipment: bool,
    calendar: bool,
    stock_purchase: bool,
    stock_consume: bool,
    stock_inventory: bool,
    stock_transfer: bool,
    stock_open: bool,
    stock_edit: bool,
    shoppinglist_items_add: bool,
    shoppinglist_items_delete: bool,
    recipes_mealplan: bool,
    chore_track_execution: bool,
    chore_undo_execution: bool,
    batteries_track_charge_cycle: bool,
    batteries_undo_charge_cycle: bool,
    tasks_undo_execution: bool,
    tasks_mark_completed: bool,
    master_data_edit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GlobalUserConfig {
    settings: UserSettings,
    id: i32,
    permission: Permissions,
    username: String,
    picture_file_name: String,
}

// https://stackoverflow.com/questions/62746540/diesel-with-custom-wrapper-types
fn action<T>(connection: PooledConnection<ConnectionManager<T>>) -> QueryResult<GlobalConfig>
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
    // usersettings.ts actually accesses this so we need to return something useful
    Ok(GlobalConfig {})
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
    *const str: FromSql<diesel::sql_types::Text, <T as diesel::Connection>::Backend>,
{
    let connection = pool.get().map_err(R2D2Error)?;
    Ok(HttpResponse::Ok()
        .json(web::block(move || action(connection).map_err(|e| e.to_string())).await?))
}
