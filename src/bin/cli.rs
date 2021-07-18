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
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "the not-grocy database migration tool")]
enum Cli {
    Migrate,
    ListMigrations,
    Rollback { version: String },
}

pub struct BarrelMigration {
    version: String,
    up: String,
    down: String,
}

// we roll our own cli because the official one creates terrible errors if the migrations have compilation errors
// and developing migrations has no good ide support.
// also switching databases is not supported.

fn main() -> Result<(), RunMigrationsError> {
    let args = Cli::from_args();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let migrations = [not_grocy_server::migrations::m20210716230021_init::BarrelMigration {}];
    println!("{:?}", connection.latest_run_migration_version()?);

    match args {
        Cli::Migrate => run_migrations(&connection, migrations, &mut std::io::stdout()),
        Cli::ListMigrations => Ok(()), // https://lib.rs/crates/dialoguer
        Cli::Rollback { version: v } => {
            let migration_to_revert = BarrelMigration {
                version: "20210716230021".to_string(),
                up: m_up.make::<Sqlite>(),
                down: m_down.make::<Sqlite>(),
            };

            connection.transaction::<_, RunMigrationsError, _>(|| {
                println!("Rolling back migration {}", migration_to_revert.version());
                migration_to_revert.revert(&connection)?;
                let target =
                    __diesel_schema_migrations.filter(version.eq(migration_to_revert.version()));
                ::diesel::delete(target).execute(&connection)?;
                Ok(())
            })
        }
    }
}
