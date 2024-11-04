use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::{create_dir_all, read_dir, File, OpenOptions};
use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::db::sled_db::SledDb;
use crate::db::{Db, DbError};
use crate::storage::interface::{StorageDataValue, StorageDir, StorageInteract};

use log::{debug, error, warn};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub const STORAGE_USER_DIR: &str = "user";
pub const STORAGE_USER_SESSION_KEY: &str = "session";
pub const STORAGE_USER_USER_SESSION_KEY: &str = "user_session";
pub const STORAGE_APP_DIR: &str = "app";
pub const SELECTED_REGION_STORAGE_KEY: &str = "countryRegionSelection";
pub const STORAGE_LOGGING_DIR: &str = "logging";

const STORAGE_HIDDEN_FILE_NAME: &str = ".store";
const CRATE_WORKSPACE: &str = "cloudcore"; //env!("CARGO_PKG_NAME"); // TODO: Using pkg name breaks backward compatibility. Need to figure out migration method
const DB_DIR: &str = "store";

#[derive(Debug, Clone)]
pub struct Storage {
    /// Contains the path constructed with
    /// the OS File directory path with our module path.
    /// Returns a reference of the Path -> Slice [u8]
    parent_path: PathBuf,
    /// Contains actual implementation. Uses Mutex to ensure atomic storage modifications.
    inner: Arc<Mutex<InnerStorage>>,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageInteractError {
    #[error("serialization or deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] Error),
    #[error("a key needs to be provided to retrieve the storage")]
    EmptyKey,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateStorageError {
    #[error("failed to open db: {0}")]
    Db(#[from] DbError),
    #[error("io error: {0}")]
    Io(#[from] Error),
}

pub type StorageInteractResult<T> = std::result::Result<T, StorageInteractError>;

#[derive(Debug, Serialize, Deserialize)]
struct StorageData {
    /// Data will hold all the persisted objects in storage. Its the entry point to our storage.
    /// ```
    /// {
    ///   "data"; {
    ///     // ... storage.
    ///   }
    /// }
    /// ```
    data: HashMap<String, Value>,
}

#[derive(Debug)]
struct InnerStorage {
    parent_path: PathBuf,
    db: Db,
}

impl Storage {
    /// In order to construct this object we need to pass in the
    /// OS file directory where we can have access to
    /// read and write to persist our data. We pass in the OS file dir
    /// as a String and then it gets formatted into a path to be consumed by the lib
    ///
    /// Don't create several instances of storage that point to the same `os_dir` by this method.
    /// It can lead to data races. Use clone() on the storage instead.
    pub fn new(os_dir: impl AsRef<Path>) -> std::result::Result<Storage, CreateStorageError> {
        let storage = InnerStorage::new(os_dir)?;

        Ok(Storage {
            parent_path: storage.parent_path.clone(),
            inner: Arc::new(Mutex::new(storage)),
        })
    }

    pub fn wipe_storage(
        &self,
        remove_app_folder: bool,
        remove_selected_region: bool,
    ) -> Result<()> {
        self.lock_storage()
            .wipe_storage(remove_app_folder, remove_selected_region)
    }

    /// Get a reference to the parent path which is the OS file dir.
    pub fn parent_path(&self) -> &Path {
        self.parent_path.as_path()
    }

    /// Returns a cloned db [SledDb].
    pub fn db(&self) -> Db {
        self.lock_storage().db.clone()
    }

    fn lock_storage(&self) -> MutexGuard<InnerStorage> {
        self.inner.lock().unwrap()
    }
}

impl InnerStorage {
    fn new(os_dir: impl AsRef<Path>) -> std::result::Result<InnerStorage, CreateStorageError> {
        let os_dir = os_dir.as_ref();
        if !fs::metadata(os_dir)?.is_dir() {
            return Err(CreateStorageError::Io(Error::new(
                ErrorKind::Other,
                "not a directory",
            )));
        }

        let mut parent_path = os_dir.to_path_buf();
        parent_path.push(CRATE_WORKSPACE);
        create_dir_all(&parent_path)?;

        let mut db_path = os_dir.to_path_buf();
        db_path.push(DB_DIR);
        create_dir_all(&db_path)?;

        let mut storage = InnerStorage {
            parent_path,
            db: Db::new(Box::new(SledDb::open(db_path)?)),
        };
        debug!("Parent Path: {:?}", storage.parent_path.as_path());

        if !storage.parent_path.join(STORAGE_USER_DIR).try_exists()? {
            match storage.make_dir_for_child(STORAGE_USER_DIR) {
                Ok(_) => debug!("storage made for user data"),
                Err(err) => error!("Could not create user data storage: {}", err),
            }
        }

        if !storage.parent_path.join(STORAGE_APP_DIR).try_exists()? {
            match storage.make_dir_for_child(STORAGE_APP_DIR) {
                Ok(_) => debug!("storage made for app data"),
                Err(err) => error!("Could not create app data storage: {}", err),
            }
        }

        // Keep this for now so it is deleted for existing users
        if storage.parent_path.join(STORAGE_LOGGING_DIR).try_exists()? {
            let _ = storage.remove_dir_for_child(STORAGE_LOGGING_DIR);
        }

        Ok(storage)
    }

    fn wipe_storage(
        &mut self,
        remove_app_folder: bool,
        remove_selected_region: bool,
    ) -> Result<()> {
        // TODO: Move country region selection to 'app' folder
        let country_region_selection =
            self.get_value(STORAGE_USER_DIR, SELECTED_REGION_STORAGE_KEY);

        let child_paths = read_dir(self.parent_path())?.filter_map(|path| match path {
            Ok(entry) => Some(entry),
            Err(err) => {
                error!("Error reading directory Entry: {err}");
                None
            }
        });

        for entry in child_paths {
            let file_name = entry.file_name();
            let path = entry.path();
            if (file_name != STORAGE_APP_DIR || remove_app_folder) && path.is_dir() {
                if let Err(err) = fs::remove_dir_all(path) {
                    error!(
                        "Error deleting child path {}: {err}",
                        <OsString as AsRef<Path>>::as_ref(&file_name).display()
                    );
                }
            }
        }

        if let (Ok(StorageDataValue::String(string)), false) =
            (country_region_selection, remove_selected_region)
        {
            let result = self.set_value(STORAGE_USER_DIR, SELECTED_REGION_STORAGE_KEY, string);
            if let Err(err) = result {
                warn!("Couldn't set selected region after wipe: {err}");
            }
        }

        Ok(())
    }

    fn make_dir_for_child(&mut self, child_dir: impl AsRef<Path>) -> Result<PathBuf> {
        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(&child_dir);
        create_dir_all(&full_path)?;

        debug!("Child Path: {:?}", &full_path);
        self.touch_file_for_child(child_dir, None)?;

        Ok(full_path)
    }

    fn remove_dir_for_child(&mut self, child_dir: impl AsRef<Path>) -> Result<()> {
        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(child_dir);
        fs::remove_dir_all(full_path)
    }

    fn touch_file_for_child(
        &mut self,
        child_dir: impl AsRef<Path>,
        bytes: Option<&str>,
    ) -> Result<()> {
        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(child_dir);
        full_path.push(STORAGE_HIDDEN_FILE_NAME);

        self.write_bytes_to_disk(&full_path, bytes.unwrap_or(r#"{"data":{}}"#).as_bytes())
    }

    fn stream_buffer_from_child(&mut self, child_dir: impl AsRef<Path>) -> Result<String> {
        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(child_dir);
        full_path.push(STORAGE_HIDDEN_FILE_NAME);
        self.read_from_disk_to_string(full_path)
    }

    fn get_value(
        &mut self,
        path: impl AsRef<Path>,
        key: &str,
    ) -> StorageInteractResult<StorageDataValue> {
        if key.is_empty() {
            return Err(StorageInteractError::EmptyKey);
        }

        let path = path.as_ref();
        let io_buffer = self.stream_buffer_from_child(path)?;
        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(path);

        let storage: StorageData = serde_json::from_str(&io_buffer)?;
        let value = storage.data.get(key).unwrap_or(&Value::Null).clone();

        Ok(value)
    }

    fn set_value<T>(
        &mut self,
        path: impl AsRef<Path>,
        key: &str,
        value: T,
    ) -> StorageInteractResult<()>
    where
        T: Serialize,
    {
        let path = path.as_ref();

        let mut full_path = self.parent_path().to_path_buf();
        full_path.push(path);
        if !full_path.try_exists()? {
            self.make_dir_for_child(path)?;
        }

        let io_buffer = self.stream_buffer_from_child(path)?;
        let mut cd: StorageData = serde_json::from_str(&io_buffer)?;
        let value = serde_json::to_value(&value)?;
        cd.data.insert(key.to_string(), value);
        let out_buffer = serde_json::to_string(&cd)?;
        self.touch_file_for_child(path, Some(&out_buffer))?;

        Ok(())
    }

    fn remove_value(&mut self, path: impl AsRef<Path>, key: &str) -> StorageInteractResult<()> {
        let path = path.as_ref();

        let io_buffer = self.stream_buffer_from_child(path)?;

        let mut cd: StorageData = serde_json::from_str(&io_buffer)?;
        if cd.data.remove(key).is_none() {
            debug!("No value for '{}' in storage", key);
        } else {
            let out_buffer = serde_json::to_string(&cd)?;
            self.touch_file_for_child(path, Some(&out_buffer))?;
        }

        Ok(())
    }

    fn init_device_storage(&mut self, dsn: &str) -> StorageInteractResult<()> {
        let mut full_dsn_path = self.parent_path().to_path_buf();
        full_dsn_path.push(dsn);
        if !full_dsn_path.try_exists()? {
            self.make_dir_for_child(dsn)?;
        }

        Ok(())
    }

    fn clear_device_storage(&mut self, dsn: &str) -> StorageInteractResult<()> {
        let mut full_dsn_path = self.parent_path().to_path_buf();
        full_dsn_path.push(dsn);
        if !full_dsn_path.try_exists()? {
            return Ok(());
        }

        self.remove_dir_for_child(dsn)?;
        Ok(())
    }

    fn parent_path(&self) -> &Path {
        self.parent_path.as_path()
    }

    fn write_bytes_to_disk(&mut self, path: &Path, bytes: &[u8]) -> Result<()> {
        if let Some(dir) = path.parent() {
            create_dir_all(dir)?;
        }
        let mut file = File::create(path)?;
        let mut content = Cursor::new(bytes);
        std::io::copy(&mut content, &mut file)?;
        file.flush()?;
        Ok(())
    }

    fn read_from_disk_to_string(&mut self, path: impl AsRef<Path>) -> Result<String> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut input_buffer = String::new();
        file.read_to_string(&mut input_buffer)?;
        Ok(input_buffer)
    }
}

// OS and File Dir Handlers
impl StorageDir for Storage {
    fn make_dir_for_child(&self, child_dir: impl AsRef<Path>) -> Result<PathBuf> {
        self.lock_storage().make_dir_for_child(child_dir)
    }

    fn remove_dir_for_child(&self, child_dir: impl AsRef<Path>) -> Result<()> {
        self.lock_storage().remove_dir_for_child(child_dir)
    }

    fn touch_file_for_child(&self, child_dir: impl AsRef<Path>, bytes: Option<&str>) -> Result<()> {
        self.lock_storage().touch_file_for_child(child_dir, bytes)
    }

    fn stream_buffer_from_child(&self, child_dir: impl AsRef<Path>) -> Result<String> {
        self.lock_storage().stream_buffer_from_child(child_dir)
    }
}

// User Facing Handlers
impl StorageInteract for Storage {
    fn get_value(
        &self,
        path: impl AsRef<Path>,
        key: &str,
    ) -> StorageInteractResult<StorageDataValue> {
        self.lock_storage().get_value(path, key)
    }

    fn set_value<T>(&self, path: impl AsRef<Path>, key: &str, value: T) -> StorageInteractResult<()>
    where
        T: Serialize,
    {
        self.lock_storage().set_value(path, key, value)
    }

    fn remove_value(&self, path: impl AsRef<Path>, key: &str) -> StorageInteractResult<()> {
        self.lock_storage().remove_value(path, key)
    }

    fn init_device_storage(&self, dsn: &str) -> StorageInteractResult<()> {
        self.lock_storage().init_device_storage(dsn)
    }

    fn clear_device_storage(&self, dsn: &str) -> StorageInteractResult<()> {
        self.lock_storage().clear_device_storage(dsn)
    }
}
