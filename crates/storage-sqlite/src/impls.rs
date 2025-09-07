use std::{marker::PhantomData, path::PathBuf};

use reth_tracing::tracing::{debug, error, info};
use rusqlite::Connection;

use crate::{SqliteStore, dao::*, schema::MIGRATIONS_DIR};

impl R for SqliteStore<Read> {
    type Mode = Read;

    fn new(db_path: PathBuf) -> rusqlite::Result<SqliteStore<Self::Mode>> {
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

impl R for SqliteStore<Write> {
    type Mode = Write;

    fn new(db_path: PathBuf) -> rusqlite::Result<SqliteStore<Self::Mode>> {
        info!("Migrations Dir: {:?}", MIGRATIONS_DIR.path());
        let mut conn = Connection::open(db_path.clone())?;

        // match MIGRATIONS.to_latest(&mut conn) {
        //     Ok(_) => (),
        //     Err(e) => {
        //         error!("Failed to apply migrations: {}", e);
        //         return rusqlite::Result::Err(rusqlite::Error::UnwindingPanic);
        //     }
        // }

        // conn.pragma_update(None, "journal_mode", String::from("WAL"))
        //     .unwrap();

        Ok(SqliteStore {
            db_path,
            conn,
            _mode: PhantomData,
        })
    }
}

impl W for SqliteStore<Write> {
    fn new(db_path: PathBuf) -> rusqlite::Result<Self> {
        let mut conn = Connection::open(db_path.clone())?;

        // match MIGRATIONS.to_latest(&mut conn) {
        //     Ok(_) => (),
        //     Err(e) => {
        //         error!("Failed to apply migrations: {}", e);
        //         return rusqlite::Result::Err(rusqlite::Error::UnwindingPanic);
        //     }
        // }

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
