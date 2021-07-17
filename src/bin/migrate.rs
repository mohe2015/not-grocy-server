use std::env;
use std::path::Path;

use barrel::backend::Sqlite;
use diesel::connection::SimpleConnection;
use diesel::migration::RunMigrationsError;
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use diesel_migrations::run_migrations;
use diesel_migrations::Migration;
use diesel_migrations::MigrationConnection;
use dotenv::dotenv;
use migrations_internals::schema::__diesel_schema_migrations::dsl::*;

pub struct BarrelMigration {
    version: String,
    up: String,
    down: String,
}

impl Migration for BarrelMigration {
    fn file_path(&self) -> Option<&Path> {
        None
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn run(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.up)?;
        Ok(())
    }

    fn revert(&self, conn: &dyn SimpleConnection) -> Result<(), RunMigrationsError> {
        conn.batch_execute(&self.down)?;
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

    let migration = BarrelMigration {
        version: "20210716230021".to_string(),
        up: m_up.make::<Sqlite>(),
        down: m_down.make::<Sqlite>(),
    };

    let migration_to_revert = BarrelMigration {
        version: "20210716230021".to_string(),
        up: m_up.make::<Sqlite>(),
        down: m_down.make::<Sqlite>(),
    };

    let migrations = [migration];

    println!("{:?}", connection.latest_run_migration_version()?);

    /*connection.transaction::<_, RunMigrationsError, _>(|| {
        println!("Rolling back migration {}", migration_to_revert.version());
        migration_to_revert.revert(&connection)?;
        let target = __diesel_schema_migrations.filter(version.eq(migration_to_revert.version()));
        ::diesel::delete(target).execute(&connection)?;
        Ok(())
    })?;*/

    run_migrations(&connection, migrations, &mut std::io::stdout())
}
