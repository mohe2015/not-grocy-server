use std::path::{Path, PathBuf};

use barrel::backend::Sqlite;
use diesel::connection::SimpleConnection;
use diesel::migration::{Migration, RunMigrationsError};
use dotenv::dotenv;

/// Represents a migration run inside Diesel
///
/// 1. Path
/// 2. Version
/// 3. Up
/// 4. Down
pub struct BarrelMigration(PathBuf, String, String, String);

impl Migration for BarrelMigration {
    fn file_path(&self) -> Option<&Path> {
        Some(self.0.as_path())
    }

    fn version(&self) -> &str {
        &self.1
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.2)?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.3)?;
        Ok(())
    }
}

fn main() {
    dotenv().ok();

    let mut m_up = barrel::Migration::new();
    not_grocy_server::migrations::m20210716230021_init::up(&mut m_up);
    m_up.make::<Sqlite>();
    let mut m_down = barrel::Migration::new();
    not_grocy_server::migrations::m20210716230021_init::down(&mut m_down);
    m_down.make::<Sqlite>();
}
