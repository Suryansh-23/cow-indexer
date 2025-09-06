use include_dir::{Dir, include_dir};
use reth_tracing::tracing::error;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use std::marker::PhantomData;
use std::sync::LazyLock;

pub mod dao;
pub mod models;
pub mod schema;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../migrations");
static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

struct Read;
struct Write;

#[derive(Debug)]
pub struct SqliteStore<Mode> {
    db_path: String,
    conn: Connection,
    _mode: PhantomData<Mode>,
}

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

impl R for SqliteStore<Read> {}

impl R for SqliteStore<Write> {}

impl W for SqliteStore<Write> {
    fn new(db_path: Option<&str>) -> rusqlite::Result<Self> {
        let db_path = match db_path {
            Some(path) => path,
            None => "cow.db",
        };

        let mut conn = Connection::open(db_path)?;

        match MIGRATIONS.to_latest(&mut conn) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to apply migrations: {}", e);
                return rusqlite::Result::Err(rusqlite::Error::UnwindingPanic);
            }
        }

        conn.pragma_update(None, "journal_mode", String::from("WAL"))
            .unwrap();

        Ok(Self {
            db_path: db_path.into(),
            conn,
            _mode: PhantomData,
        })
    }

    fn read_only(&self) -> rusqlite::Result<SqliteStore<Read>> {
        let conn =
            Connection::open_with_flags(&self.db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        Ok(SqliteStore {
            db_path: self.db_path.clone(),
            conn,
            _mode: PhantomData,
        })
    }
}
