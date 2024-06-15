use eyre::Result;
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Abi {
    pub(crate) id: u64,
    pub(crate) name: String,
}

pub(crate) struct AbiRepository {
    pub(crate) connection: Arc<Mutex<Connection>>,
}

impl AbiRepository {
    pub(crate) fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    pub(crate) async fn get_by_id(&self, id: i64) -> Result<Option<Abi>> {
        let connection = self.connection.lock().await;
        let mut statement = connection.prepare("SELECT * FROM abis WHERE id = ?1;")?;
        let row = statement
            .query_row([id], |row| {
                Ok(Abi {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .optional()?;
        Ok(row)
    }

    pub(crate) async fn list(&self) -> Result<Vec<Abi>> {
        let connection = self.connection.lock().await;
        let mut statement = connection.prepare("SELECT * FROM abis;")?;
        let rows = statement.query_map([], |row| {
            Ok(Abi {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        let result = rows.map(|row| row.unwrap()).collect::<Vec<Abi>>();
        Ok(result)
    }

    pub(crate) async fn create(&self, name: String) -> Result<i64> {
        let connection = self.connection.lock().await;
        let row = connection
            .prepare("INSERT INTO abis (name) VALUES (?1) RETURNING rowid;")?
            .query_row([name], |row| Ok(row.get::<_, i64>(0)))?;
        match row {
            Ok(row_id) => Ok(row_id),
            Err(_) => eyre::bail!("Failed to insert ABI"),
        }
    }
}
