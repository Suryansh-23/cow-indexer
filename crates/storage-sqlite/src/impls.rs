use std::marker::PhantomData;

use reth_tracing::tracing::error;
use rusqlite::Connection;

use crate::{SqliteStore, dao::*, schema::MIGRATIONS};

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
