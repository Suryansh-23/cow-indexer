use std::marker::PhantomData;
use std::sync::LazyLock;

use include_dir::{Dir, include_dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;

use crate::SqliteStore;

pub struct Read;
pub struct Write;

pub trait R {
    fn new(db_path: Option<&str>) -> rusqlite::Result<SqliteStore<Read>> {
        let db_path = match db_path {
            Some(path) => path,
            None => "cow.db",
        };

        let conn =
            Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        Ok(SqliteStore {
            db_path: db_path.into(),
            conn,
            _mode: PhantomData,
        })
    }
}
pub trait W: R {
    fn new(db_path: Option<&str>) -> rusqlite::Result<SqliteStore<Write>>;
    fn read_only(&self) -> rusqlite::Result<SqliteStore<Read>>;
}
