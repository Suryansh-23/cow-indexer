use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;
use std::sync::LazyLock;

pub static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../migrations");
// pub static MIGRATIONS: LazyLock<Migrations<'static>> =
//     LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());
