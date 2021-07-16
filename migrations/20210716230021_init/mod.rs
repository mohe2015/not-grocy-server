/// Handle up migrations 
fn up(migr: &mut Migration) {
    migr.create_table("api_keys", |t| {
        t.add_column("id", types::integer().increments(true).primary(true));
        t.add_column("api_key", types::text().unique(true));
        t.add_column("user_id", types::integer());
        t.add_column("expires", types::custom("DATETIME").default(types::WrappedDefault::Custom("NOW()")));
    });
} 

/// Handle down migrations 
fn down(migr: &mut Migration) {
    // TODO FIXME remove later to prevent data loss
    migr.drop_table("api_keys");
} 
