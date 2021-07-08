# not-grocy-server

# Development

I personally recommend to use [rustup](https://www.rust-lang.org/tools/install) and [VSCodium](https://vscodium.com/#install) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

Also the Diesel CLI may be useful:
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

Specify a database in the `.env` file:
```
DATABASE_URL=development.db
```