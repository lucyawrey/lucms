# LuCMS

Experimental Rust-based headless CMS.

## Requirements
- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- [SQLx CLI](https://crates.io/crates/sqlx-cli)

## Getting Started
- Make sure the rust Toolchain is installed. On Linux and WSL you can do this with the command:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Clone the repo.
- Follow the SQLx setup steps below to make sure you have a working PostgreSQL database.
- Make sure you have a `.env` file containing the variables `DATABASE_URL` and `HOST` or else the program won't run. As of now, this is checked in as it contains no secrets.
- Run `cargo run` to start the project.

## SQLite Database Setup with SQLx

The SQLx CLI requires libssl, if you don't already have it you can install it with the following command on Ubuntu:
```sh
sudo apt-get install libssl-dev
```

Next, install the SQLx CLI with Cargo if you don't already have it:
```sh
cargo install sqlx-cli
```

Run migrations to create and properly configure necessary tables:
```sh
sqlx migrate run
```
