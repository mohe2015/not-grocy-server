use std::{marker::PhantomData, path::Path};

use barrel::{backend::SqlGenerator, types::*};
use diesel::connection::SimpleConnection;
use diesel_migrations::{Migration, RunMigrationsError};

pub struct BarrelMigration<T: SqlGenerator> {
    pub phantom_data: PhantomData<T>,
}

impl<T: SqlGenerator> Migration for BarrelMigration<T> {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        "2"
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let mut migr = barrel::Migration::new();

        migr.change_table("stock", |t| {
            t.add_column("new_opened_date", date().nullable(true));
        });

        migr.inject_custom("UPDATE stock SET new_opened_date = opened_date");

        migr.change_table("stock", |t| {
            // TODO sqlite3 supports this - fix barrel (https://www.sqlite.org/lang_altertable.html)
            // t.drop_column("opened_date");
            t.inject_custom("DROP COLUMN opened_date");
        });

        migr.change_table("stock", |t| {
            // TODO sqlite3 supports this - fix barrel (https://www.sqlite.org/lang_altertable.html)
            t.inject_custom("RENAME COLUMN new_opened_date TO opened_date");
            // t.rename_column("new_opened_date", "opened_date");
        });

        println!("{}", &migr.make::<T>());
        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let migr = barrel::Migration::new();

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
