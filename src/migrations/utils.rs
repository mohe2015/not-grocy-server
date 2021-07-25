use barrel::{functions::AutogenFunction, types::*, Migration, Table};

pub fn id(t: &mut Table) {
    t.add_column("id", integer().increments(true).primary(true));
}

pub fn created(t: &mut Table) {
    t.add_column(
        "row_created_timestamp",
        datetime()
            .nullable(true)
            .default(AutogenFunction::CurrentTimestamp),
    );
}

pub fn undone(t: &mut Table) {
    t.add_column("undone", boolean().default(false));
    t.add_column("undone_timestamp", datetime().nullable(true));
}

pub fn name(t: &mut Table) {
    t.add_column("name", text().unique(true));
}

pub fn description(t: &mut Table) {
    t.add_column("description", text().nullable(true));
}

pub fn create_or_update<F>(migr: &mut Migration, table_name: &str, cb: &'static F)
where
    F: 'static + Fn(&mut Table) -> (),
{
    migr.create_table_if_not_exists(format!("new_{}", table_name), cb);

    // TO prevent errors if it didn't exist
    migr.create_table_if_not_exists(format!("{}", table_name), cb);

    migr.inject_custom(format!(
        "INSERT INTO new_{} SELECT * FROM {}",
        table_name, table_name
    ));

    migr.drop_table_if_exists(table_name);

    migr.rename_table(format!("new_{}", table_name), table_name.to_string());
}
