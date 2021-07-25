use std::marker::PhantomData;

use barrel::{
    backend::{Pg, SqlGenerator, Sqlite},
    functions::AutogenFunction,
    types::*,
    Migration, Table,
};

pub fn id(t: &mut Table) {
    t.add_column("id", integer().increments(true).primary(true));
}

pub fn created(t: &mut Table) {
    t.add_column(
        "row_created_timestamp",
        datetime()
            .nullable(true)
            .default(AutogenFunction::CurrentTimestamp),
    );
}

pub fn undone(t: &mut Table) {
    t.add_column("undone", boolean().default(false));
    t.add_column("undone_timestamp", datetime().nullable(true));
}

pub fn name(t: &mut Table) {
    t.add_column("name", text().unique(true));
}

pub fn description(t: &mut Table) {
    t.add_column("description", text().nullable(true));
}

pub fn id2() -> (&'static str, barrel::types::Type) {
    ("id", integer().increments(true).primary(true))
}

pub fn created2() -> (&'static str, barrel::types::Type) {
    (
        "row_created_timestamp",
        datetime()
            .nullable(true)
            .default(AutogenFunction::CurrentTimestamp),
    )
}

pub fn undone2() -> (&'static str, barrel::types::Type) {
    ("undone", boolean().default(false))
}

pub fn undone_timestamp2() -> (&'static str, barrel::types::Type) {
    ("undone_timestamp", datetime().nullable(true))
}

pub fn name2() -> (&'static str, barrel::types::Type) {
    ("name", text().unique(true))
}

pub fn description2() -> (&'static str, barrel::types::Type) {
    ("description", text().nullable(true))
}

pub fn create_or_update<F>(migr: &mut Migration, table_name: &str, cb: &'static F)
where
    F: 'static + Fn(&mut Table),
{
    migr.create_table_if_not_exists(format!("new_{}", table_name), cb);

    // TO prevent errors if it didn't exist
    migr.create_table_if_not_exists(table_name.to_string(), cb);

    migr.inject_custom(format!(
        "INSERT INTO new_{} SELECT * FROM {}",
        table_name, table_name
    ));

    //migr.inject_custom(format!("ALTER TABLE {} DISABLE TRIGGER ALL", table_name));

    migr.drop_table_if_exists(table_name);

    migr.rename_table(format!("new_{}", table_name), table_name.to_string());
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
            for (column_name, column_type) in test.call(()) {
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
            for (column_name, column_type) in test.call(()) {
                t.add_column(column_name, column_type.clone());
            }
        });

        // TO prevent errors if it didn't exist
        migr.create_table_if_not_exists(table_name.to_string(), move |t| {
            for (column_name, column_type) in test.call(()) {
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
