use crate::db::{BucketEngine, Bytes, DbEngine, DbError, DbResult, Pair};
use sled::Tree;
use std::path::Path;

/// A key-value db using sled as its storage engine.
#[derive(Debug, Clone)]
pub struct SledDb {
    db: sled::Db,
}

/// [BucketEngine] implementation using sled.
/// The data is flushed on drop.
#[derive(Debug, Clone)]
pub struct SledBucketEngine {
    tree: Tree,
}

impl SledDb {
    /// Opens a Db at the specified path. This will create a new storage directory at the specified path if it does not already exist.
    /// The same directory shouldn't be used for multiple DBs at the same time. If you want multiple instances of the same DB, you should clone it.
    pub fn open(path: impl AsRef<Path>) -> DbResult<Self> {
        let db = sled::open(path)?;

        Ok(SledDb { db })
    }
}

impl DbEngine for SledDb {
    fn open_bucket(&self, id: &str) -> DbResult<Box<dyn BucketEngine>> {
        let tree = self.db.open_tree(id.as_bytes())?;
        Ok(Box::new(SledBucketEngine { tree }))
    }

    fn delete_bucket(&self, id: &str) -> DbResult<bool> {
        self.db.drop_tree(id.as_bytes()).map_err(DbError::from)
    }
}

impl BucketEngine for SledBucketEngine {
    fn get(&self, key: &[u8]) -> DbResult<Option<Bytes>> {
        let bytes = self.tree.get(key)?;
        Ok(bytes.map(|bytes| Box::new(bytes) as Bytes))
    }

    fn insert(&self, key: &[u8], value: &[u8]) -> DbResult<Option<Bytes>> {
        let bytes = self.tree.insert(key, value)?;
        Ok(bytes.map(|bytes| Box::new(bytes) as Bytes))
    }

    fn remove(&self, key: &[u8]) -> DbResult<Option<Bytes>> {
        let bytes = self.tree.remove(key)?;
        Ok(bytes.map(|bytes| Box::new(bytes) as Bytes))
    }

    fn iter(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Pair>>>> {
        let iter = self
            .tree
            .iter()
            .map(|res| res.map_err(DbError::from))
            .map(|res| res.map(|(key, value)| (Box::new(key) as Bytes, Box::new(value) as Bytes)));

        Ok(Box::new(iter))
    }

    fn keys(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Bytes>>>> {
        let iter = self
            .tree
            .iter()
            .keys()
            .map(|res| res.map_err(DbError::from))
            .map(|res| res.map(|bytes| Box::new(bytes) as Bytes));

        Ok(Box::new(iter))
    }

    fn values(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Bytes>>>> {
        let iter = self
            .tree
            .iter()
            .values()
            .map(|res| res.map_err(DbError::from))
            .map(|res| res.map(|bytes| Box::new(bytes) as Bytes));

        Ok(Box::new(iter))
    }

    fn clear(&self) -> DbResult<()> {
        self.tree.clear().map_err(DbError::from)
    }
}

impl From<sled::Error> for DbError {
    fn from(err: sled::Error) -> Self {
        DbError::DbEngineError(Box::new(err))
    }
}

impl Drop for SledDb {
    fn drop(&mut self) {
        // Auto-flushing may not work as expected on some platforms.
        // Related issue: https://github.com/spacejam/sled/issues/1328
        let _ = self.db.flush();
    }
}

impl Drop for SledBucketEngine {
    fn drop(&mut self) {
        // Auto-flushing may not work as expected on some platforms.
        // Related issue: https://github.com/spacejam/sled/issues/1328
        let _ = self.tree.flush();
    }
}
