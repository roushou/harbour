pub(crate) mod abis;

use eyre::Result;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

use self::abis::AbiRepository;

pub(crate) struct Database {
    pub(crate) abis: AbiRepository,
}

impl Database {
    pub(crate) fn builder() -> DatabaseBuilder {
        DatabaseBuilder::new()
    }
}

pub(crate) struct DatabaseBuilder {}

impl DatabaseBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn initialize(self) -> Result<Database> {
        let connection = Connection::open_in_memory()?;
        connection.execute(
            "CREATE TABLE abis (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            (),
        )?;
        let connection = Arc::new(Mutex::new(connection));
        Ok(Database {
            abis: AbiRepository::new(connection),
        })
    }
}
