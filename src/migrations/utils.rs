use barrel::{functions::AutogenFunction, types::*, Table};

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
