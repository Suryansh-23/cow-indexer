// Data-Access-Object
use crate::SqliteStore;
use rusqlite::Connection;
use std::{marker::PhantomData, path::PathBuf};

pub struct Read;
pub struct Write;

pub trait R {
    fn new(db_path: PathBuf) -> rusqlite::Result<SqliteStore<Read>> {
        let conn = Connection::open_with_flags(
            db_path.clone(),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;

        Ok(SqliteStore {
            db_path,
            conn,
            _mode: PhantomData,
        })
    }
}
pub trait W: R {
    fn new(db_path: Option<&str>) -> rusqlite::Result<SqliteStore<Write>>;
    fn read_only(&self) -> rusqlite::Result<SqliteStore<Read>>;
}
