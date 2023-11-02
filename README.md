# LuCMS

Experimental Rust-based headless CMS.

## Requirements
- [Rust Toolchain](https://www.rust-lang.org/tools/install)

## Getting Started
- Clone the repo.
- Follow the steps below to make sure you have a working `lucms.db` SQLite file.
- Run `cargo run` to start the project.

## SQLite Database Setup with SQLx

Install SQLx CLI with Cargo:
```sh
cargo install sqlx-cli
```

Create an empty `lucms.db` database file:
```sh
sqlx database create
```

Run migrations to create and properly configure necessary tables:
```sh
sqlx migrate run
```
