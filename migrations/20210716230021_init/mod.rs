/// Handle up migrations 
fn up(migr: &mut Migration) {
    migr.create_table("api_keys", |t| {
        t.add_column("id", types::integer().increments(true).primary(true));
        t.add_column("api_key", types::text().unique(true));
        t.add_column("user_id", types::integer());
        // TODO FIXME this doesnt work because the internally generated thing uses it's own barrel?
        // see https://git.irde.st/spacekookie/barrel/-/blob/main/src/integrations/diesel.rs
        // maybe instead write the code above manually
        t.add_column("expires", types::datetime());
    });
} 

/// Handle down migrations 
fn down(migr: &mut Migration) {
    // TODO FIXME remove later to prevent data loss
    migr.drop_table("api_keys");
} 
