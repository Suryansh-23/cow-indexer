// Data-Access-Object
use crate::SqliteStore;
use std::path::PathBuf;

pub struct Read;
pub struct Write;

pub trait R {
    type Mode;

    fn new(db_path: PathBuf) -> rusqlite::Result<SqliteStore<Self::Mode>>;
}
pub trait W: R {
    fn new(db_path: PathBuf) -> rusqlite::Result<SqliteStore<Write>>;
    fn read_only(&self) -> rusqlite::Result<SqliteStore<Read>>;
}
