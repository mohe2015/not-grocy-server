# not-grocy-server

If you are using Nix:
```
nix develop
```
otherwise install dependencies manually.

I personally recommend to use [rustup](https://www.rust-lang.org/tools/install) and [VSCodium](https://vscodium.com/#install) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

```
cargo install diesel_cli
```

Specify a database in the `.env` file:
```
DATABASE_URL=mysql://user@127.0.0.1:15306/commerce
#DATABASE_URL=mysql://not-grocy:not-grocy@not-grocy/not-grocy
#DATABASE_URL=postgres://not-grocy:not-grocy@not-grocy/not-grocy
#DATABASE_URL=test.db
```

You can copy your old grocy database file if you have one.

Run migrations:
```
cargo run --bin cli migrate
diesel print-schema > src/schema.rs
```

Run:
```
cargo run --bin server
```

# Development

Add pre-commit hook:
```bash
ln -s ../../pre-commit.sh .git/hooks/pre-commit
```

```bash
cargo install cargo-watch
RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo watch -x 'run --bin server'
```

```
cargo +nightly -Z unstable-options clippy --fix
```

# What the experience currently feels like

https://rust-lang.github.io/wg-async-foundations/vision/submitted_stories/status_quo/alan_picks_web_server.html