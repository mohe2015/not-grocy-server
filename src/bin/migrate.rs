use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let database_url = database::database_url(matches);
    let dir = migrations_dir(matches).unwrap_or_else(handle_error);
    let dir = FileBasedMigrations::from_path(dir).unwrap_or_else(handle_error);
    call_with_conn!(database_url, run_migrations_with_output(dir))?;
    regenerate_schema_if_file_specified(matches)?;

    Ok()
}
