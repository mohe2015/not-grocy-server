use barrel::{
    backend::{MySql, Pg, Sqlite},
    functions::AutogenFunction,
    types::*,
    Migration,
};

pub fn id2() -> (&'static str, barrel::types::Type) {
    ("id", integer().increments(true).primary(true))
}

pub fn created2() -> (&'static str, barrel::types::Type) {
    (
        "row_created_timestamp",
        datetime().default(AutogenFunction::CurrentTimestamp),
    )
}

pub fn undone2() -> (&'static str, barrel::types::Type) {
    ("undone", boolean().default(false))
}

pub fn undone_timestamp2() -> (&'static str, barrel::types::Type) {
    ("undone_timestamp", datetime().nullable(true))
}

pub fn name2() -> (&'static str, barrel::types::Type) {
    ("name", text())
}

pub fn description2() -> (&'static str, barrel::types::Type) {
    ("description", text().nullable(true))
}

pub trait DatabaseDependentMigrationCommands {
    fn database_dependent_migration(_migr: &mut Migration) {}
}

impl DatabaseDependentMigrationCommands for Pg {}

impl DatabaseDependentMigrationCommands for MySql {}

impl DatabaseDependentMigrationCommands for Sqlite {
    fn database_dependent_migration(migr: &mut Migration) {
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

        migr.inject_custom("DROP INDEX IF EXISTS ix_batteries_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_chores_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_product_barcodes");
        migr.inject_custom("DROP INDEX IF EXISTS ix_products_performance1");
        migr.inject_custom("DROP INDEX IF EXISTS ix_products_performance2");
        migr.inject_custom("DROP INDEX IF EXISTS ix_recipes");
        migr.inject_custom("DROP INDEX IF EXISTS ix_stock_performance1");
    }
}

pub trait CreateOrUpdate {
    fn create_or_update2(
        migr: &mut Migration,
        table_name: &str,
        test: &'static dyn Fn() -> Vec<(&'static str, barrel::types::Type)>,
    );
}

impl CreateOrUpdate for Pg {
    fn create_or_update2(
        migr: &mut Migration,
        table_name: &str,
        test: &'static dyn Fn() -> Vec<(&'static str, barrel::types::Type)>,
    ) {
        migr.create_table_if_not_exists(table_name.to_string(), move |t| {
            for (column_name, column_type) in test() {
                t.add_column(column_name, column_type.clone());
            }
        });

        // TODO FIXME implement change_column (for postgres)
        /*migr.change_table(table_name.to_string(), move |t| {
            for (column_name, column_type) in test.call(()) {
                t.change_column(column_name, column_type.clone());
            }
        });*/
    }
}

impl CreateOrUpdate for MySql {
    fn create_or_update2(
        migr: &mut Migration,
        table_name: &str,
        test: &'static dyn Fn() -> Vec<(&'static str, barrel::types::Type)>,
    ) {
        migr.create_table_if_not_exists(table_name.to_string(), move |t| {
            for (column_name, column_type) in test() {
                t.add_column(column_name, column_type.clone());
            }
        });

        // TODO FIXME implement change_column (for postgres)
        /*migr.change_table(table_name.to_string(), move |t| {
            for (column_name, column_type) in test.call(()) {
                t.change_column(column_name, column_type.clone());
            }
        });*/
    }
}

impl CreateOrUpdate for Sqlite {
    fn create_or_update2(
        migr: &mut Migration,
        table_name: &str,
        test: &'static dyn Fn() -> Vec<(&'static str, barrel::types::Type)>,
    ) {
        migr.create_table_if_not_exists(format!("new_{}", table_name), move |t| {
            for (column_name, column_type) in test() {
                t.add_column(column_name, column_type.clone());
            }
        });

        // TO prevent errors if it didn't exist
        migr.create_table_if_not_exists(table_name.to_string(), move |t| {
            for (column_name, column_type) in test() {
                t.add_column(column_name, column_type.clone());
            }
        });

        migr.inject_custom(format!(
            "INSERT INTO new_{} SELECT * FROM {}",
            table_name, table_name
        ));

        //migr.inject_custom(format!("ALTER TABLE {} DISABLE TRIGGER ALL", table_name));

        migr.drop_table_if_exists(table_name);

        migr.rename_table(format!("new_{}", table_name), table_name.to_string());
    }
}
