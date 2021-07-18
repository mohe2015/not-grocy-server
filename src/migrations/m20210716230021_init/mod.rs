use barrel::{types, Migration};

pub struct BarrelMigration<T>;

impl Migration for BarrelMigration<T> {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        "20210716230021"
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let migr = barrel::Migration::new();
        migr.create_table("api_keys", |t| {
            t.add_column("id", types::integer().increments(true).primary(true));
            t.add_column("api_key", types::text().unique(true));
            t.add_column("user_id", types::integer());
            // TODO FIXME this doesnt work because the internally generated thing uses it's own barrel?
            // see https://git.irde.st/spacekookie/barrel/-/blob/main/src/integrations/diesel.rs
            // maybe instead write the code above manually
            t.add_column("expires", types::datetime());
        });

        conn.batch_execute(migr.make::<T>())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        let migr = barrel::Migration::new();
        // TODO FIXME remove later to prevent data loss
        migr.drop_table("api_keys");

        conn.batch_execute(migr.make::<T>())
    }
}
