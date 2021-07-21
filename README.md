# not-grocy-server

# Development

If you are using Nix:
```
nix develop --command fish
```
otherwise install dependencies manually.

I personally recommend to use [rustup](https://www.rust-lang.org/tools/install) and [VSCodium](https://vscodium.com/#install) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

Also the Diesel CLI may be useful:
```bash
cargo install diesel_cli --features="barrel-migrations,barrel/sqlite3,sqlite"
```

Specify a database in the `.env` file:
```
DATABASE_URL=development.db
# DATABASE_URL=postgres://not-grocy:not-grocy@not-grocy/not-grocy
```

Currently you need to copy the database file from grocy.

Add pre-commit hook:
```bash
ln -s ../../pre-commit.sh .git/hooks/pre-commit
```

Run migrations:
```bash
~/.cargo/bin/diesel migration run
```

Generate migration:
```bash
~/.cargo/bin/diesel migration generate setup --format="barrel"
```

## Barrel docs

https://docs.rs/barrel/0.6.5/barrel/migration/struct.Migration.html
https://docs.rs/barrel/0.6.5/barrel/table/struct.Table.html
https://docs.rs/barrel/0.6.5/barrel/types/index.html
https://docs.rs/barrel/0.6.5/barrel/types/struct.Type.html

If you get
```
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', barrel-0.6.5/src/integrations/diesel.rs:182:29
```
you probably have a syntax error in your migration file.

```
cargo +nightly build -Ztimings
```

```bash
cargo run --bin cli rollback 1 && RUST_BACKTRACE=1 cargo run --bin cli migrate
~/.cargo/bin/diesel print-schema > src/schema.rs

cargo install cargo-watch
cargo watch -x 'run --bin server'
```
