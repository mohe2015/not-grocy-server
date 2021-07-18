use std::{marker::PhantomData, path::Path};

use barrel::{backend::SqlGenerator, functions::AutogenFunction, types::*};
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
            t.add_column("id", integer().increments(true).primary(true));
            t.add_column("api_key", text().unique(true));
            t.add_column("user_id", integer());
            t.add_column(
                "expires",
                datetime()
                    .nullable(true)
                    .default(AutogenFunction::CurrentTimestamp),
            );
            t.add_column("last_used", datetime().nullable(true));
            t.add_column(
                "row_created_timestamp",
                datetime().default(AutogenFunction::CurrentTimestamp),
            );
            t.add_column("key_type", text().default("default"));
        });

        migr.create_table("batteries", |t| {
            t.add_column("id", integer().increments(true).primary(true));
        });

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();
        // TODO FIXME remove later to prevent data loss
        migr.drop_table("api_keys");

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
