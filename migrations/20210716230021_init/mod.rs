/// Handle up migrations 
fn up(migr: &mut Migration) {
    migr.create_table("api_keys", |t| {
        t.add_column("id", types::integer().increments(true).unique(true));
        t.add_column("api_key", types::varchar(255));
        t.add_column("age", types::integer());
        t.add_column("owns_plushy_sharks", types::boolean());
    });
} 

/// Handle down migrations 
fn down(migr: &mut Migration) {
    // TODO FIXME remove later to prevent data loss
    migr.drop_table("api_keys");
} 
