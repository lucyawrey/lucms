# LuCMS

## SQLite Database Setup with SQLx

Install SQLx CLI with Cargo:
```sh
cargo install sqlx-cli
```

Create empty database:
```sh
sqlx database create
```

Run migrations:
```sh
sqlx migrate run
```