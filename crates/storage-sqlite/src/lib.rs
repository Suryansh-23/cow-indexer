use rusqlite::Connection;
use std::{marker::PhantomData, path::PathBuf};

pub mod dao;
pub mod impls;
pub mod models;
pub mod schema;

#[derive(Debug)]
pub struct SqliteStore<Mode> {
    db_path: PathBuf,
    conn: Connection,
    _mode: PhantomData<Mode>,
}
