use std::env;
use std::path::Path;

use barrel::backend::Sqlite;
use diesel::connection::SimpleConnection;
use diesel::migration::{Migration, RunMigrationsError};
use diesel::Connection;
use diesel::SqliteConnection;
use dotenv::dotenv;

/// Represents a migration run inside Diesel
///
/// 1. Path
/// 2. Version
/// 3. Up
/// 4. Down
pub struct BarrelMigration(String, String, String);

impl Migration for BarrelMigration {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        &self.0
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.1)?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.2)?;
        Ok(())
    }
}

fn main() -> Result<(), RunMigrationsError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let mut m_up = barrel::Migration::new();
    not_grocy_server::migrations::m20210716230021_init::up(&mut m_up);
    println!("{}", m_up.make::<Sqlite>());
    let mut m_down = barrel::Migration::new();
    not_grocy_server::migrations::m20210716230021_init::down(&mut m_down);
    println!("{}", m_down.make::<Sqlite>());

    let migration = BarrelMigration(
        "20210716230021".to_string(),
        m_up.make::<Sqlite>(),
        m_down.make::<Sqlite>(),
    );

    migration.run(&connection)
}
