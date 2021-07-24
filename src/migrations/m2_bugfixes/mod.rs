use std::{marker::PhantomData, path::Path};

use barrel::{backend::SqlGenerator, functions::AutogenFunction, types::*};
use diesel::connection::SimpleConnection;
use diesel_migrations::{Migration, RunMigrationsError};

use crate::migrations::utils::*;

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

        // TODO put this in a common method for changing column type
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

        // https://www.sqlite.org/lang_altertable.html#otheralter

        // TODO FIXME restrict foreign key deletions

        // copied from m1_init
        migr.create_table_if_not_exists("new_stock", |t| {
            id(t);
            t.add_column("product_id", foreign("products", "id")); // CHANGED
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
            t.add_column("opened_date", datetime().nullable(true));
            created(t);
            t.add_column("location_id", integer().nullable(true));
            t.add_column("shopping_location_id", integer().nullable(true));
        });

        migr.inject_custom("INSERT INTO new_stock SELECT * FROM stock");

        migr.drop_table("stock");

        migr.rename_table("new_stock", "stock");

        //println!("{}", &migr.make::<T>());
        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let migr = barrel::Migration::new();

        conn.batch_execute(&migr.make::<T>())?;
        Ok(())
    }
}
