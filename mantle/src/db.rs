use log::warn;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

pub mod sled_db;

/// A trait for a db that can create typed buckets.
pub trait DbEngine: Debug + Send + Sync + 'static {
    /// Opens or creates a new [Bucket].
    fn open_bucket(&self, id: &str) -> DbResult<Box<dyn BucketEngine>>;

    /// Removes [Bucket] from the disk.
    fn delete_bucket(&self, id: &str) -> DbResult<bool>;
}

/// A trait for a key-value bucket that only supports bytes.
pub trait BucketEngine: Debug + Send + Sync + 'static {
    /// Retrieves a value from the bucket if it exists.
    fn get(&self, key: &[u8]) -> DbResult<Option<Bytes>>;

    /// Inserts a key-value pair to the bucket, returning the old value if it was set.
    fn insert(&self, key: &[u8], value: &[u8]) -> DbResult<Option<Bytes>>;

    /// Removes a key from the bucket, returning the value at the key if the key was previously in the bucket.
    fn remove(&self, key: &[u8]) -> DbResult<Option<Bytes>>;

    /// Returns an iterator over all key-value pairs in the bucket.
    /// Order is arbitrary.
    fn iter(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Pair>>>>;

    /// Returns an iterator over all keys in the bucket.
    fn keys(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Bytes>>>>;

    /// Returns an iterator over all values in the bucket.
    fn values(&self) -> DbResult<Box<dyn Iterator<Item = DbResult<Bytes>>>>;

    /// Removes all values from the bucket.
    fn clear(&self) -> DbResult<()>;
}

// Avoid an unnecessary memory copy of bytes.
pub type Bytes = Box<dyn AsRef<[u8]>>;
pub type DbResult<T> = Result<T, DbError>;
pub type Pair = (Bytes, Bytes);

/// A key-value DB that can create typed buckets.
/// You do not have to wrap the DB in an Rc or Arc to reuse it, because it already uses an Arc internally. Just clone it.
/// Cloned db will point to the same data.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let _ = std::fs::remove_dir_all("db");
/// use mantle_utilities::db::{Bucket, Db, DbEngine};
/// use mantle_utilities::db::sled_db::SledDb;
///
/// let db_engine_implementation = SledDb::open("db")?;
/// let db = Db::new(Box::new(db_engine_implementation));
/// // A typed bucket ensures you'll write those types and not others at compile time.
/// let bucket: Bucket<String, i32> = db.open_bucket("bucket_id")?;
/// let key = "MyKey".to_string();
/// let value = 42;
/// bucket.insert(&key, &value)?;
/// assert_eq!(Some(value), bucket.get(&key)?);
///
/// # let _ = std::fs::remove_dir_all("db");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Db {
    engine: Arc<dyn DbEngine>,
}

/// A bucket that supports typed key/value pairs.
/// A bucket represents a single logical keyspace.
/// Key-value pairs stored in a [BucketEngine] as bytes. A [Bucket] uses bincode to (de)serialize them.
///
/// # Supported Types
///
/// The bucket can store any type that implements the [serde::Serialize] and [serde::Deserialize] traits with one exception.
/// Types that use deserialize_any aren't supported (e.g. untagged enums).
#[derive(Debug, Clone)]
pub struct Bucket<K, V> {
    engine: Arc<dyn BucketEngine>,
    _marker: PhantomData<(K, V)>,
}

/// Iterator over key-value pairs in a [Bucket].
pub struct Iter<K, V> {
    engine_iter: Box<dyn Iterator<Item = DbResult<Pair>>>,
    _marker: PhantomData<(K, V)>,
}

/// Iterator over keys in a [Bucket].
pub struct KeysIter<K> {
    engine_iter: Box<dyn Iterator<Item = DbResult<Bytes>>>,
    _marker: PhantomData<K>,
}

/// Iterator over values in a [Bucket].
pub struct ValuesIter<V> {
    engine_iter: Box<dyn Iterator<Item = DbResult<Bytes>>>,
    _marker: PhantomData<V>,
}

/// An Error type encapsulates all possible errors in a [DbEngine].
#[derive(thiserror::Error, Debug)]
pub enum DbError {
    /// Bincode (de)serialization error
    #[error("(de)serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    /// Implementation specific error.
    #[error(transparent)]
    DbEngineError(#[from] Box<dyn Error + Send + Sync + 'static>),
}

impl Db {
    /// Creates a new engine instance backed by the `engine` implementation.
    pub fn new(engine: Box<dyn DbEngine>) -> Self {
        Db {
            engine: engine.into(),
        }
    }

    /// Opens or creates a new [Bucket].
    pub fn open_bucket<K, V>(&self, id: impl AsRef<str>) -> DbResult<Bucket<K, V>>
    where
        K: Serialize + DeserializeOwned,
        V: Serialize + DeserializeOwned,
    {
        let bucket_engine = self.engine.open_bucket(id.as_ref())?;
        Ok(Bucket::new(bucket_engine))
    }

    /// Removes [Bucket] from the disk.
    pub fn delete_bucket(&self, id: impl AsRef<str>) -> DbResult<bool> {
        self.engine.delete_bucket(id.as_ref())
    }
}

impl<K, V> Bucket<K, V>
where
    K: Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    /// Retrieves a value from the [Bucket] if it exists.
    pub fn get(&self, key: &K) -> DbResult<Option<V>> {
        let key_encoded = bincode::serialize(key)?;
        let value_encoded = self.engine.get(&key_encoded)?;
        if let Some(value_enc) = value_encoded {
            let value = bincode::deserialize(value_enc.as_ref().as_ref())?;
            return Ok(Some(value));
        }

        Ok(None)
    }

    /// Inserts a key-value pair to the [Bucket], returning the old value if it was set.
    pub fn insert(&self, key: &K, value: &V) -> DbResult<Option<V>> {
        let key_encoded = bincode::serialize(key)?;
        let value_encoded = bincode::serialize(value)?;
        let old_value = self.engine.insert(&key_encoded, &value_encoded)?;
        Ok(self.deserialize_old_value(old_value))
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the [Bucket].
    pub fn remove(&self, key: &K) -> DbResult<Option<V>> {
        let key_encoded = bincode::serialize(key)?;
        let old_value = self.engine.remove(&key_encoded)?;
        Ok(self.deserialize_old_value(old_value))
    }

    /// Returns an iterator over all key-value pairs in the [Bucket].
    /// Order is arbitrary.
    pub fn iter(&self) -> DbResult<Iter<K, V>> {
        let engine_iter = self.engine.iter()?;
        Ok(Iter {
            engine_iter,
            _marker: Default::default(),
        })
    }

    /// Returns an iterator over all keys in the [Bucket].
    /// Order is arbitrary.
    pub fn keys(&self) -> DbResult<KeysIter<K>> {
        let engine_iter = self.engine.keys()?;
        Ok(KeysIter {
            engine_iter,
            _marker: Default::default(),
        })
    }

    /// Returns an iterator over all values in the [Bucket].
    /// Order is arbitrary.
    pub fn values(&self) -> DbResult<ValuesIter<V>> {
        let engine_iter = self.engine.values()?;
        Ok(ValuesIter {
            engine_iter,
            _marker: Default::default(),
        })
    }

    /// Removes all values from the [Bucket].
    pub fn clear(&self) -> DbResult<()> {
        self.engine.clear().map_err(DbError::from)
    }

    /// Creates a new bucket with the provided implementation.
    fn new(engine: Box<dyn BucketEngine>) -> Self {
        Bucket {
            engine: engine.into(),
            _marker: Default::default(),
        }
    }

    fn deserialize_old_value(&self, old_value: Option<Bytes>) -> Option<V> {
        match old_value.map(|old| bincode::deserialize(old.as_ref().as_ref())) {
            Some(Ok(value)) => Some(value),
            Some(Err(err)) => {
                warn!("failed to deserialize the old value: {err}");
                None
            }
            None => None,
        }
    }
}

impl<K, V> Iterator for Iter<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    type Item = DbResult<(K, V)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.engine_iter.next().map(|result| {
            let (enc_key, enc_value) = result?;
            let key = bincode::deserialize(enc_key.as_ref().as_ref())?;
            let value = bincode::deserialize(enc_value.as_ref().as_ref())?;
            Ok((key, value))
        })
    }
}

impl<K> Iterator for KeysIter<K>
where
    K: DeserializeOwned,
{
    type Item = DbResult<K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.engine_iter.next().map(|result| {
            let enc_key = result?;
            let key = bincode::deserialize(enc_key.as_ref().as_ref())?;
            Ok(key)
        })
    }
}

impl<V> Iterator for ValuesIter<V>
where
    V: DeserializeOwned,
{
    type Item = DbResult<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.engine_iter.next().map(|result| {
            let enc_key = result?;
            let key = bincode::deserialize(enc_key.as_ref().as_ref())?;
            Ok(key)
        })
    }
}
