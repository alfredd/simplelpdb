use rocksdb::{DB, Options};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("RocksDB error: {0}")]
    RocksError(#[from] rocksdb::Error),
    #[error("Value not found for key: {0}")]
    NotFound(String),
}
#[derive(Debug)]
pub struct DbWrapper {
    db: DB,
}

impl DbWrapper {
    /// Open a new or existing RocksDB database
    pub fn new(path: &str) -> Result<Self, DBError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }

    /// Open a new or existing in-memory RocksDB database
    pub fn new_in_memory() -> Result<Self, DBError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, "memory://in_memory_db")?;
        Ok(Self { db })
    }

    /// Write a key-value pair to the database
    pub fn write(&self, key: &str, value: &str) -> Result<(), DBError> {
        self.db.put(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    /// Read a value from the database given a key
    pub fn read(&self, key: &str) -> String {
        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => String::from_utf8_lossy(&value).into_owned(),
            Ok(None) => String::new(),
            Err(_) => {log::error!("Error while reading from the database");String::new()},
        }
    }

    /// Delete a key-value pair from the database
    pub fn delete(&self, key: &str) -> Result<(), DBError> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }
}

impl Default for DbWrapper {
    fn default() -> Self {
        Self::new("_default_rocksdb_storage").expect("Failed to create default RocksDBWrapper")
    }
}