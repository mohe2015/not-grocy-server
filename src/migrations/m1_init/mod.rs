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
        datetime().default(AutogenFunction::CurrentTimestamp),
    );
}

fn undone(t: &mut Table) {
    t.add_column("undone", boolean().default(false));
    t.add_column("undone_timestamp", datetime().nullable(true));
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
            t.add_column("name", text().unique(true));
            t.add_column("description", text().nullable(true));
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
            t.add_column("name", text().unique(true));
            t.add_column("description", text().nullable(true));
            t.add_column("period_type", text());
            t.add_column("period_days", integer().nullable(true));
            created(t);
            t.add_column("period_config", text().nullable(true));
            t.add_column("track_date_only", boolean().default(false));
            t.add_column("rollover", boolean().default(false));
            t.add_column("assignment_type", text().nullable(true));
            t.add_column("assignment_config", text().nullable(true));
            t.add_column(
                "next_execution_assigned_to_user_id",
                integer().nullable(true),
            );
            t.add_column("consume_product_on_execution", boolean().default(false));
            t.add_column("product_id", integer().nullable(true));
            t.add_column("product_amount", double().nullable(true));
            t.add_column("period_interval", integer().default(1));
            t.add_column("active", boolean().default(true));
        });

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();
        // TODO FIXME remove later to prevent data loss
        migr.drop_table_if_exists("api_keys");
        migr.drop_table_if_exists("batteries");

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
