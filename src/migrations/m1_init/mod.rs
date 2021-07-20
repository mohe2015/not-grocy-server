use std::{marker::PhantomData, path::Path};

use barrel::{backend::SqlGenerator, functions::AutogenFunction, types::*, Table};
use diesel::connection::SimpleConnection;
use diesel_migrations::{Migration, RunMigrationsError};

pub struct BarrelMigration<T: SqlGenerator> {
    pub phantom_data: PhantomData<T>,
}

/*
pub struct DateTimeNow<T: SqlGenerator>;

impl From<DateTimeNow<Pg>> for WrappedDefault<'static> {
    fn from(value: DateTimeNow) -> Self {
        WrappedDefault::Function(value)
    }
}

impl From<DateTimeNow<Sqlite>> for WrappedDefault<'static> {
    fn from(value: DateTimeNow) -> Self {
        WrappedDefault::Function(value)
    }
}
*/

fn id(t: &mut Table) {
    t.add_column("id", integer().increments(true).primary(true));
}

fn created(t: &mut Table) {
    t.add_column(
        "row_created_timestamp",
        datetime()
            .nullable(true)
            .default(AutogenFunction::CurrentTimestamp),
    );
}

fn undone(t: &mut Table) {
    t.add_column("undone", boolean().default(false));
    t.add_column("undone_timestamp", datetime().nullable(true));
}

fn name(t: &mut Table) {
    t.add_column("name", text().unique(true));
}

fn description(t: &mut Table) {
    t.add_column("description", text().nullable(true));
}

impl<T: SqlGenerator> Migration for BarrelMigration<T> {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        "1"
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();
        migr.create_table("api_keys", |t| {
            id(t);
            t.add_column("api_key", text().unique(true));
            t.add_column("user_id", integer());
            t.add_column(
                "expires",
                datetime()
                    .nullable(true)
                    .default(AutogenFunction::CurrentTimestamp),
            );
            t.add_column("last_used", datetime().nullable(true));
            created(t);
            t.add_column("key_type", text().default("default"));
        });

        migr.create_table("batteries", |t| {
            id(t);
            name(t);
            description(t);
            t.add_column("used_in", text().nullable(true));
            t.add_column("charge_interval_days", integer().default(0));
            created(t);
            t.add_column("active", boolean().default(true));
        });

        migr.create_table("battery_charge_cycles", |t| {
            id(t);
            t.add_column("battery_id", text());
            t.add_column("tracked_time", datetime().nullable(true));
            created(t);
            undone(t);
        });

        migr.create_table("chores", |t| {
            id(t);
            name(t);
            description(t);
            t.add_column("period_type", text());
            t.add_column("period_days", integer().nullable(true));
            created(t);
            t.add_column("period_config", text().nullable(true));
            t.add_column("track_date_only", boolean().nullable(true).default(false));
            t.add_column("rollover", boolean().nullable(true).default(false));
            t.add_column("assignment_type", text().nullable(true));
            t.add_column("assignment_config", text().nullable(true));
            t.add_column(
                "next_execution_assigned_to_user_id",
                integer().nullable(true),
            );
            t.add_column("consume_product_on_execution", boolean().default(false));
            t.add_column("product_id", integer().nullable(true));
            t.add_column("product_amount", float().nullable(true));
            t.add_column("period_interval", integer().default(1));
            t.add_column("active", boolean().default(true));
        });

        migr.create_table("chores_log", |t| {
            id(t);
            t.add_column("chore_id", integer());
            t.add_column("tracked_time", datetime().nullable(true));
            t.add_column("done_by_user_id", integer().nullable(true));
            created(t);
            undone(t);
        });

        migr.create_table("equipment", |t| {
            id(t);
            name(t);
            description(t);
            t.add_column("instruction_manual_file_name", text().nullable(true));
            created(t);
        });

        migr.create_table("locations", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
            t.add_column("is_freezer", boolean().default(false));
        });

        migr.create_table("meal_plan", |t| {
            id(t);
            t.add_column("day", date());
            t.add_column("type", text().nullable(true).default("recipe"));
            t.add_column("recipe_id", integer().nullable(true));
            t.add_column("recipe_servings", integer().nullable(true).default(1));
            t.add_column("note", text().nullable(true));
            t.add_column("product_id", integer().nullable(true));
            t.add_column("product_amount", float().nullable(true).default(0));
            t.add_column("product_qu_id", integer().nullable(true));
            created(t);
        });

        migr.create_table("permission_hierarchy", |t| {
            id(t);
            name(t);
            t.add_column("parent", integer().nullable(true));
        });

        migr.create_table("product_barcodes", |t| {
            id(t);
            t.add_column("product_id", integer());
            t.add_column("barcode", text());
            t.add_column("qu_id", integer().nullable(true));
            t.add_column("amount", float().nullable(true));
            t.add_column("shopping_location_id", integer().nullable(true));
            t.add_column("last_price", double().nullable(true)); // DECIMAL
            created(t);
            t.add_column("note", text().nullable(true));
        });

        migr.create_table("product_groups", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
        });

        migr.create_table("products", |t| {
            id(t);
            name(t);
            description(t);
            t.add_column("product_group_id", integer().nullable(true));
            t.add_column("active", boolean().default(true));
            t.add_column("location_id", integer());
            t.add_column("shopping_location_id", integer().nullable(true));
            t.add_column("qu_id_purchase", integer());
            t.add_column("qu_id_stock", integer());
            t.add_column("qu_factor_purchase_to_stock", float());
            t.add_column("min_stock_amount", integer().default(0));
            t.add_column("default_best_before_days", integer().default(0));
            t.add_column("default_best_before_days_after_open", integer().default(0));
            t.add_column(
                "default_best_before_days_after_freezing",
                integer().default(0),
            );
            t.add_column(
                "default_best_before_days_after_thawing",
                integer().default(0),
            );
            t.add_column("picture_file_name", text().nullable(true));
            t.add_column("enable_tare_weight_handling", boolean().default(false));
            t.add_column("tare_weight", float().default(0));
            t.add_column(
                "not_check_stock_fulfillment_for_recipes",
                boolean().default(false).nullable(true),
            );
            t.add_column("parent_product_id", integer().nullable(true));
            t.add_column("calories", integer().nullable(true));
            t.add_column(
                "cumulate_min_stock_amount_of_sub_products",
                boolean().default(false).nullable(true),
            );
            t.add_column("due_type", integer().default(1));
            t.add_column("quick_consume_amount", float().default(1));
            t.add_column("hide_on_stock_overview", boolean().default(false));
            created(t);
            t.add_column("default_print_stock_label", integer().default(0));
            t.add_column("allow_label_per_unit", integer().default(0));
        });

        migr.create_table("quantity_unit_conversions", |t| {
            id(t);
            t.add_column("from_qu_id", integer());
            t.add_column("to_qu_id", integer());
            t.add_column("factor", float());
            t.add_column("product_id", integer().nullable(true));
            created(t);
        });

        migr.create_table("quantity_units", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
            t.add_column("name_plural", text().nullable(true));
            t.add_column("plural_forms", text().nullable(true));
        });

        migr.create_table("recipes", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
            t.add_column("picture_file_name", text().nullable(true));
            t.add_column("base_servings", integer().nullable(true).default(1));
            t.add_column("desired_servings", integer().nullable(true).default(1));
            t.add_column("not_check_shoppinglist", boolean().default(false));
            t.add_column("type", text().nullable(true).default("normal"));
            t.add_column("product_id", integer().nullable(true));
        });

        migr.create_table("recipes_nestings", |t| {
            id(t);
            t.add_column("recipe_id", integer());
            t.add_column("includes_recipe_id", integer());
            created(t);
            t.add_column("servings", integer().default(1).nullable(true));
        });

        migr.create_table("recipes_pos", |t| {
            id(t);
            t.add_column("recipe_id", integer());
            t.add_column("product_id", integer());
            t.add_column("amount", float().default(0));
            t.add_column("note", text().nullable(true));
            t.add_column("qu_id", integer().nullable(true));
            t.add_column("only_check_single_unit_in_stock", boolean().default(false));
            t.add_column("ingredient_group", text().nullable(true));
            t.add_column("not_check_stock_fulfillment", boolean().default(false));
            created(t);
            t.add_column("variable_amount", text().nullable(true));
            t.add_column("price_factor", float().default(1));
        });

        migr.create_table("sessions", |t| {
            id(t);
            t.add_column("session_key", text().unique(true));
            t.add_column("user_id", integer());
            t.add_column("expires", datetime().nullable(true));
            t.add_column("last_used", datetime().nullable(true));
            created(t);
        });

        migr.create_table("shopping_list", |t| {
            id(t);
            t.add_column("product_id", integer().nullable(true));
            t.add_column("note", text().nullable(true));
            t.add_column("amount", double().default(0)); // DECIMAL
            created(t);
            t.add_column("shopping_list_id", integer().nullable(true).default(1));
            t.add_column("done", boolean().nullable(true).default(false));
            t.add_column("qu_id", integer().nullable(true));
        });

        migr.create_table("shopping_lists", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
        });

        migr.create_table("shopping_locations", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
        });

        migr.create_table("stock", |t| {
            id(t);
            t.add_column("product_id", integer());
            t.add_column("amount", double()); // DECIMAL
            t.add_column("best_before_date", date().nullable(true));
            t.add_column(
                "purchased_date",
                date()
                    .nullable(true)
                    .default(AutogenFunction::CurrentTimestamp),
            );
            t.add_column("stock_id", text());
            t.add_column("price", double().nullable(true)); // DECIMAL
            t.add_column("open", boolean().default(false));
            created(t);
            t.add_column("location_id", integer().nullable(true));
            t.add_column("shopping_location_id", integer().nullable(true));
            t.add_column("opened_date", datetime().nullable(true));
        });

        migr.create_table("stock_log", |t| {
            id(t);
            t.add_column("product_id", integer());
            t.add_column("amount", double()); // DECIMAL
            t.add_column("best_before_date", date().nullable(true));
            t.add_column("purchased_date", date().nullable(true));
            t.add_column("used_date", date().nullable(true));
            t.add_column("spoiled", boolean().default(false));
            t.add_column("stock_id", text());
            t.add_column("transaction_type", text());
            t.add_column("price", double().nullable(true)); // DECIMAL
            t.add_column("undone", boolean().default(false));
            t.add_column("undone_timestamp", datetime().nullable(true));
            t.add_column("opened_date", datetime().nullable(true));
            created(t);
            t.add_column("location_id", integer().nullable(true));
            t.add_column("recipe_id", integer().nullable(true));
            t.add_column("correlation_id", text().nullable(true));
            t.add_column("transaction_id", text().nullable(true));
            t.add_column("stock_row_id", integer().nullable(true));
            t.add_column("shopping_location_id", integer().nullable(true));
            t.add_column("user_id", integer().default(1));
        });

        migr.create_table("task_categories", |t| {
            id(t);
            name(t);
            description(t);
            created(t);
        });

        migr.create_table("tasks", |t| {
            id(t);
            name(t);
            description(t);
            t.add_column("due_date", datetime().nullable(true));
            t.add_column("done", boolean().default(false));
            t.add_column("done_timestamp", datetime().nullable(true));
            t.add_column("category_id", integer().nullable(true));
            t.add_column("assigned_to_user_id", integer().nullable(true));
            created(t);
        });

        migr.create_table("user_permissions", |t| {
            id(t);
            t.add_column("permission_id", integer());
            t.add_column("user_id", integer());
        });

        migr.create_table("user_settings", |t| {
            id(t);
            t.add_column("user_id", integer());
            t.add_column("key", text());
            t.add_column("value", text().nullable(true));
            created(t);
        });

        migr.create_table("userentities", |t| {
            id(t);
            name(t);
            t.add_column("caption", text());
            description(t);
            t.add_column("show_in_sidebar_menu", boolean().default(true));
            t.add_column("icon_css_class", text().nullable(true));
            created(t);
        });

        migr.create_table("userfield_values", |t| {
            id(t);
            t.add_column("field_id", integer());
            t.add_column("object_id", integer());
            t.add_column("value", text());
            created(t);
        });

        migr.create_table("userfields", |t| {
            id(t);
            t.add_column("entity", text());
            name(t);
            t.add_column("caption", text());
            t.add_column("type", text());
            t.add_column("show_as_column_in_tables", boolean().default(false));
            created(t);
            t.add_column("config", text().nullable(true));
            t.add_column("sort_number", integer().nullable(true));
        });

        migr.create_table("userobjects", |t| {
            id(t);
            t.add_column("userentity_id", integer());
            created(t);
        });

        migr.create_table("users", |t| {
            id(t);
            t.add_column("username", text().unique(true));
            t.add_column("first_name", text().nullable(true));
            t.add_column("last_name", text().nullable(true));
            t.add_column("password", text());
            created(t);
            t.add_column("picture_file_name", text().nullable(true));
        });

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();
        // TODO FIXME remove later to prevent data loss
        migr.drop_table_if_exists("api_keys");
        migr.drop_table_if_exists("batteries");
        migr.drop_table_if_exists("battery_charge_cycles");
        migr.drop_table_if_exists("chores");
        migr.drop_table_if_exists("chores_log");
        migr.drop_table_if_exists("equipment");
        migr.drop_table_if_exists("locations");
        migr.drop_table_if_exists("meal_plan");
        migr.drop_table_if_exists("permission_hierarchy");
        migr.drop_table_if_exists("product_barcodes");
        migr.drop_table_if_exists("product_groups");
        migr.drop_table_if_exists("products");
        migr.drop_table_if_exists("quantity_unit_conversions");
        migr.drop_table_if_exists("quantity_units");
        migr.drop_table_if_exists("recipes");
        migr.drop_table_if_exists("recipes_nestings");
        migr.drop_table_if_exists("recipes_pos");
        migr.drop_table_if_exists("sessions");
        migr.drop_table_if_exists("shopping_list");
        migr.drop_table_if_exists("shopping_lists");
        migr.drop_table_if_exists("shopping_locations");
        migr.drop_table_if_exists("stock");
        migr.drop_table_if_exists("stock_log");
        migr.drop_table_if_exists("task_categories");
        migr.drop_table_if_exists("tasks");
        migr.drop_table_if_exists("user_permissions");
        migr.drop_table_if_exists("user_settings");
        migr.drop_table_if_exists("userentities");
        migr.drop_table_if_exists("userfield_values");
        migr.drop_table_if_exists("userfields");
        migr.drop_table_if_exists("userobjects");
        migr.drop_table_if_exists("users");

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
