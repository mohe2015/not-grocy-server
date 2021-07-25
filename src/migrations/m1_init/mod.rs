use std::{marker::PhantomData, path::Path};

use barrel::{backend::SqlGenerator, functions::AutogenFunction, types::*};
use diesel::connection::SimpleConnection;
use diesel_migrations::{Migration, RunMigrationsError};

use super::utils::*;

pub struct BarrelMigration<T: SqlGenerator> {
    pub phantom_data: PhantomData<T>,
}

impl<T: SqlGenerator + CreateOrUpdate> Migration for BarrelMigration<T> {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        "1"
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();

        // warning - not backwards compatible
        migr.drop_table_if_exists("migrations");

        migr.inject_custom("DROP VIEW IF EXISTS batteries_current");
        migr.inject_custom("DROP VIEW IF EXISTS chores_assigned_users_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS chores_current");
        migr.inject_custom("DROP VIEW IF EXISTS chores_execution_users_statistics");
        migr.inject_custom("DROP VIEW IF EXISTS permission_tree");
        migr.inject_custom("DROP VIEW IF EXISTS product_barcodes_comma_separated");
        migr.inject_custom("DROP VIEW IF EXISTS product_price_history");
        migr.inject_custom("DROP VIEW IF EXISTS product_qu_relations");
        migr.inject_custom("DROP VIEW IF EXISTS products_average_price");
        migr.inject_custom("DROP VIEW IF EXISTS products_last_purchased");
        migr.inject_custom("DROP VIEW IF EXISTS products_oldest_stock_unit_price");
        migr.inject_custom("DROP VIEW IF EXISTS products_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS products_view");
        migr.inject_custom("DROP VIEW IF EXISTS quantity_unit_conversions_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS quantity_units_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS recipes_nestings_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS recipes_pos_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS recipes_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS stock_average_product_shelf_life");
        migr.inject_custom("DROP VIEW IF EXISTS stock_current");
        migr.inject_custom("DROP VIEW IF EXISTS stock_current_location_content");
        migr.inject_custom("DROP VIEW IF EXISTS stock_current_locations");
        migr.inject_custom("DROP VIEW IF EXISTS stock_missing_products");
        migr.inject_custom("DROP VIEW IF EXISTS stock_missing_products_including_opened");
        migr.inject_custom("DROP VIEW IF EXISTS tasks_current");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_shopping_list");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_stock_current_overview");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_stock_current_overview_including_opened");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_stock_journal");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_stock_journal_summary");
        migr.inject_custom("DROP VIEW IF EXISTS uihelper_user_permissions");
        migr.inject_custom("DROP VIEW IF EXISTS user_permissions_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS userfield_values_resolved");
        migr.inject_custom("DROP VIEW IF EXISTS users_dto");

        /*
        // should be dropped when the tables are dropped anyways and syntax doesn't match :(
        migr.inject_custom("DROP TRIGGER IF EXISTS cascade_battery_removal");
        migr.inject_custom("DROP TRIGGER IF EXISTS cascade_chore_removal");
        migr.inject_custom("DROP TRIGGER IF EXISTS cascade_product_removal");
        migr.inject_custom("DROP TRIGGER IF EXISTS create_internal_recipe");
        migr.inject_custom("DROP TRIGGER IF EXISTS enforce_parent_product_id_null_when_empty_INS");
        migr.inject_custom("DROP TRIGGER IF EXISTS enforce_parent_product_id_null_when_empty_UPD");
        migr.inject_custom("DROP TRIGGER IF EXISTS enfore_product_nesting_level");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_empty_userfields_INS");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_empty_userfields_UPD");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_infinite_nested_recipes_INS");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_infinite_nested_recipes_UPD");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_qu_stock_change_after_first_purchase");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_self_nested_recipes_INS");
        migr.inject_custom("DROP TRIGGER IF EXISTS prevent_self_nested_recipes_UPD");
        migr.inject_custom(
            "DROP TRIGGER IF EXISTS quantity_unit_conversions_custom_unique_constraint_INS",
        );
        migr.inject_custom(
            "DROP TRIGGER IF EXISTS quantity_unit_conversions_custom_unique_constraint_UPD",
        );
        migr.inject_custom("DROP TRIGGER IF EXISTS recipes_pos_qu_id_default");
        migr.inject_custom("DROP TRIGGER IF EXISTS remove_internal_recipe");
        migr.inject_custom("DROP TRIGGER IF EXISTS remove_items_from_deleted_shopping_list");
        migr.inject_custom("DROP TRIGGER IF EXISTS remove_recipe_from_meal_plans");
        migr.inject_custom("DROP TRIGGER IF EXISTS set_products_default_location_if_empty_stock");
        migr.inject_custom(
            "DROP TRIGGER IF EXISTS set_products_default_location_if_empty_stock_log",
        );
        migr.inject_custom("DROP TRIGGER IF EXISTS shopping_list_qu_id_default");
        */

        migr.inject_custom("DROP INDEX IF EXISTS ix_batteries_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_chores_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_product_barcodes");
        migr.inject_custom("DROP INDEX IF EXISTS ix_products_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_products_performance2");
        migr.inject_custom("DROP INDEX IF EXISTS ix_recipes");
        migr.inject_custom("DROP INDEX IF EXISTS ix_stock_performance1");

        static API_KEYS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
                id2(),
                ("api_key", text().unique(true)),
                ("user_id", integer()),
                (
                    "expires",
                    datetime()
                        .nullable(true)
                        .default(AutogenFunction::CurrentTimestamp),
                ),
                ("last_used", datetime().nullable(true)),
                created2(),
                ("key_type", text().default("default")),
            ]
        };

        T::create_or_update2(&mut migr, "api_keys", &API_KEYS_FN);

        static BATTERIES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
                id2(),
                name2(),
                description2(),
                ("used_in", text().nullable(true)),
                ("charge_interval_days", integer().default(0)),
                created2(),
                ("active", boolean().default(true)),
            ]
        };

        T::create_or_update2(&mut migr, "batteries", &BATTERIES_FN);

        static BATTERY_CHARGE_CYCLES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("battery_id", text()),
            ("tracked_time", datetime().nullable(true)),
            created2(),
            undone2(),
        ]};

        T::create_or_update2(&mut migr, "battery_charge_cycles", &BATTERY_CHARGE_CYCLES_FN);

        static CHORES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            ("period_type", text()),
            ("period_days", integer().nullable(true)),
            created2(),
            ("period_config", text().nullable(true)),
            ("track_date_only", boolean().nullable(true).default(false)),
            ("rollover", boolean().nullable(true).default(false)),
            ("assignment_type", text().nullable(true)),
            ("assignment_config", text().nullable(true)),
            (
                "next_execution_assigned_to_user_id",
                integer().nullable(true),
            ),
            ("consume_product_on_execution", boolean().default(false)),
            ("product_id", boolean().nullable(true)), // integer()
            ("product_amount", float().nullable(true)),
            ("period_interval", integer().default(1)),
            ("active", boolean().default(true)),
            ]};

        T::create_or_update2(&mut migr, "chores", &CHORES_FN);

        static CHORES_LOG_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("chore_id", integer()),
            ("tracked_time", datetime().nullable(true)),
            ("done_by_user_id", integer().nullable(true)),
            created2(),
            undone2(),
            ]};

        T::create_or_update2(&mut migr, "chores_log", &CHORES_LOG_FN);

        static EQUIPMENT_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            ("instruction_manual_file_name", text().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "equipment", &EQUIPMENT_FN);

        static LOCATIONS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ("is_freezer", boolean().default(false)),
            ]};

        T::create_or_update2(&mut migr, "locations", &LOCATIONS_FN);

        static MEAL_PLAN_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("day", date()),
            ("type", text().nullable(true).default("recipe")),
            ("recipe_id", integer().nullable(true)),
            ("recipe_servings", integer().nullable(true).default(1)),
            ("note", text().nullable(true)),
            ("product_id", integer().nullable(true)),
            ("product_amount", float().nullable(true).default(0)),
            ("product_qu_id", integer().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "meal_plan", &MEAL_PLAN_FN);

        static PERMISSION_HIERARCHY_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            ("parent", integer().nullable(true)),
            ]};


        T::create_or_update2(&mut migr, "permission_hierarchy", &PERMISSION_HIERARCHY_FN);

        static PRODUCT_BARCODES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("product_id", integer()),
            ("barcode", text()),
            ("qu_id", integer().nullable(true)),
            ("amount", float().nullable(true)),
            ("shopping_location_id", integer().nullable(true)),
            ("last_price", double().nullable(true)), // DECIMAL
            created2(),
            ("note", text().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "product_barcodes", &PRODUCT_BARCODES_FN);

        static PRODUCT_GROUPS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "product_groups", &PRODUCT_GROUPS_FN);

        static PRODUCTS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            ("product_group_id", integer().nullable(true)),
            ("active", boolean().default(true)),
            ("location_id", integer()),
            ("shopping_location_id", integer().nullable(true)),
            ("qu_id_purchase", foreign("quantity_units", "id")),
            ("qu_id_stock", foreign("quantity_units", "id")),
            ("qu_factor_purchase_to_stock", float()),
            ("min_stock_amount", integer().default(0)),
            ("default_best_before_days", integer().default(0)),
            ("default_best_before_days_after_open", integer().default(0)),
            (
                "default_best_before_days_after_freezing",
                integer().default(0),
            ),
            (
                "default_best_before_days_after_thawing",
                integer().default(0),
            ),
            ("picture_file_name", text().nullable(true)),
            ("enable_tare_weight_handling", boolean().default(false)),
            ("tare_weight", float().default(0)),
            (
                "not_check_stock_fulfillment_for_recipes",
                boolean().default(false).nullable(true),
            ),
            ("parent_product_id", integer().nullable(true)),
            ("calories", integer().nullable(true)),
            (
                "cumulate_min_stock_amount_of_sub_products",
                boolean().default(false).nullable(true),
            ),
            ("due_type", boolean().default(true)), // integer()
            ("quick_consume_amount", float().default(1)),
            ("hide_on_stock_overview", boolean().default(false)),
            created2(),
            ("default_print_stock_label", integer().default(0)),
            ("allow_label_per_unit", integer().default(0)),
            ]};

        T::create_or_update2(&mut migr, "products", &PRODUCTS_FN);

        static QUANTITY_UNIT_CONVERSIONS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("from_qu_id", integer()),
            ("to_qu_id", integer()),
            ("factor", float()),
            ("product_id", integer().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "quantity_unit_conversions", &QUANTITY_UNIT_CONVERSIONS_FN);

        static QUANTITY_UNITS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ("name_plural", text().nullable(true)),
            ("plural_forms", text().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "quantity_units", &QUANTITY_UNITS_FN);

        static RECIPES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ("picture_file_name", text().nullable(true)),
            ("base_servings", integer().nullable(true).default(1)),
            ("desired_servings", integer().nullable(true).default(1)),
            ("not_check_shoppinglist", boolean().default(false)),
            ("type", text().nullable(true).default("normal")),
            ("product_id", integer().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "recipes", &RECIPES_FN);

        static RECIPES_NESTINGS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("recipe_id", integer()),
            ("includes_recipe_id", integer()),
            created2(),
            ("servings", integer().default(1).nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "recipes_nestings", &RECIPES_NESTINGS_FN);

        static RECIPES_POS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("recipe_id", integer()),
            ("product_id", integer()),
            ("amount", float().default(0)),
            ("note", text().nullable(true)),
            ("qu_id", integer().nullable(true)),
            ("only_check_single_unit_in_stock", boolean().default(false)),
            ("ingredient_group", text().nullable(true)),
            ("not_check_stock_fulfillment", boolean().default(false)),
            created2(),
            ("variable_amount", text().nullable(true)),
            ("price_factor", float().default(1)),
            ]};

        T::create_or_update2(&mut migr, "recipes_pos", &RECIPES_POS_FN);

        static SESSIONS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("session_key", text().unique(true)),
            ("user_id", integer()),
            ("expires", datetime().nullable(true)),
            ("last_used", datetime().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "sessions", &SESSIONS_FN);

        static SHOPPING_LIST_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("product_id", integer().nullable(true)),
            ("note", text().nullable(true)),
            ("amount", double().default(0)), // DECIMAL
            created2(),
            ("shopping_list_id", integer().nullable(true).default(1)),
            ("done", integer().nullable(true).default(false)), // boolean()
            ("qu_id", integer().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "shopping_list", &SHOPPING_LIST_FN);

        static SHOPPING_LISTS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "shopping_lists", &SHOPPING_LISTS_FN);

        static SHOPPING_LOCATIONS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "shopping_locations", &SHOPPING_LOCATIONS_FN);

        static STOCK_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("product_id", foreign("products", "id")), // CHANGED
            ("amount", double()), // DECIMAL
            ("best_before_date", date().nullable(true)),
            (
                "purchased_date",
                date()
                    .nullable(true)
                    .default(AutogenFunction::CurrentTimestamp),
            ),
            ("stock_id", text()),
            ("price", double().nullable(true)), // DECIMAL
            ("open", boolean().default(false)),
            ("opened_date", date().nullable(true)),
            created2(),
            ("location_id", integer().nullable(true)),
            ("shopping_location_id", integer().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "stock", &STOCK_FN);

        static STOCK_LOG_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("product_id", integer()),
            ("amount", double()), // DECIMAL
            ("best_before_date", date().nullable(true)),
            ("purchased_date", date().nullable(true)),
            ("used_date", date().nullable(true)),
            ("spoiled", integer().default(false)), // boolean()
            ("stock_id", text()),
            ("transaction_type", text()),
            ("price", double().nullable(true)), // DECIMAL
            ("undone", boolean().default(false)),
            ("undone_timestamp", datetime().nullable(true)),
            ("opened_date", datetime().nullable(true)),
            created2(),
            ("location_id", integer().nullable(true)),
            ("recipe_id", integer().nullable(true)),
            ("correlation_id", text().nullable(true)),
            ("transaction_id", text().nullable(true)),
            ("stock_row_id", integer().nullable(true)),
            ("shopping_location_id", integer().nullable(true)),
            ("user_id", integer().default(1)),
            ]};

        T::create_or_update2(&mut migr, "stock_log", &STOCK_LOG_FN);

        static TASK_CATEGORIES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "task_categories", &TASK_CATEGORIES_FN);

        static TASKS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            description2(),
            ("due_date", datetime().nullable(true)),
            ("done", boolean().default(false)),
            ("done_timestamp", datetime().nullable(true)),
            ("category_id", integer().nullable(true)),
            ("assigned_to_user_id", integer().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "tasks", &TASKS_FN);

        static USER_PERMISSIONS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("permission_id", integer()),
            ("user_id", integer()),
            ]};

        T::create_or_update2(&mut migr, "user_permissions", &USER_PERMISSIONS_FN);

        static USER_SETTINGS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("user_id", integer()),
            ("key", text()),
            ("value", text().nullable(true)),
            created2(),
            (
                "row_updated_timestamp",
                datetime()
                    .nullable(true)
                    .default(AutogenFunction::CurrentTimestamp),
            ),
            ]};

        T::create_or_update2(&mut migr, "user_settings", &USER_SETTINGS_FN);

        static USERENTITIES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            name2(),
            ("caption", text()),
            description2(),
            ("show_in_sidebar_menu", boolean().default(true)),
            ("icon_css_class", text().nullable(true)),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "userentities", &USERENTITIES_FN);

        static USERFIELD_VALUES_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("field_id", integer()),
            ("object_id", integer()),
            ("value", text()),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "userfield_values", &USERFIELD_VALUES_FN);

        static USERFIELDS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("entity", text()),
            name2(),
            ("caption", text()),
            ("type", text()),
            ("show_as_column_in_tables", boolean().default(false)),
            created2(),
            ("config", text().nullable(true)),
            ("sort_number", integer().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "userfields", &USERFIELDS_FN);

        static USEROBJECTS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("userentity_id", integer()),
            created2(),
            ]};

        T::create_or_update2(&mut migr, "userobjects", &USEROBJECTS_FN);

        static USERS_FN: fn() -> Vec<(&'static str, barrel::types::Type)> = || {
            vec![
            id2(),
            ("username", text().unique(true)),
            ("first_name", text().nullable(true)),
            ("last_name", text().nullable(true)),
            ("password", text());
            created2(),
            ("picture_file_name", text().nullable(true)),
            ]};

        T::create_or_update2(&mut migr, "users", &USERS_FN);

        println!("{}", &migr.make::<T>());
        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let migr = barrel::Migration::new();

        /*
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
        */

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
