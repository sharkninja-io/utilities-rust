use std::io::Result;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::storage::json_storage::StorageInteractResult;

pub trait StorageInteract {
    /// Retrieve data contents and serde_json::Value which returns
    /// a object casted as a Value which can be casted to its necessary type.
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use mantle_utilities::storage::{StorageInteract, StorageInteractResult, StorageDataValue};
    /// # use std::path::Path;
    /// # struct MockStorage;
    /// # impl StorageInteract for MockStorage {
    /// #     fn get_value(&self, path: impl AsRef<Path>, key: &str) -> StorageInteractResult<StorageDataValue> {
    /// #         Ok(StorageDataValue::Null)
    /// #     }
    /// #     fn set_value<T>(&self, path: impl AsRef<Path>, key: &str, value: T) -> StorageInteractResult<()> {
    /// #         Ok(())
    /// #     }
    /// #     fn remove_value(&self, path: impl AsRef<Path>, key: &str) -> StorageInteractResult<()> {
    /// #         Ok(())
    /// #     }
    /// #     fn init_device_storage(&self, dsn: &str) -> StorageInteractResult<()> {
    /// #         Ok(())
    /// #     }
    /// #     fn clear_device_storage(&self, dsn: &str) -> StorageInteractResult<()> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// # let storage = MockStorage;
    /// let value = storage.get_value("path", "key")?;
    /// println!("{:?}", value);
    /// # Ok(())
    /// # }
    /// ```
    /// It is completely safe to unwrap since it can return None which avoids the lib to panic.
    fn get_value(
        &self,
        path: impl AsRef<Path>,
        key: &str,
    ) -> StorageInteractResult<StorageDataValue>;

    /// Set data to the mutable content. You can pass in any type which can be
    /// serde::Serialize since it needs to be converted at some point to
    /// serde_json::Value to be stored.
    fn set_value<T>(
        &self,
        path: impl AsRef<Path>,
        key: &str,
        value: T,
    ) -> StorageInteractResult<()>
    where
        T: Serialize;

    /// Remove the value for the key at the specified path. If the path does not exist
    /// an error is returned. If the key does not exist it is a NoOp
    fn remove_value(&self, path: impl AsRef<Path>, key: &str) -> StorageInteractResult<()>;

    /// Checks if storage directory exists for the given DSN and creates it if
    /// it does not exist.
    fn init_device_storage(&self, dsn: &str) -> StorageInteractResult<()>;

    /// Removes the storage directory for the given DSN.
    fn clear_device_storage(&self, dsn: &str) -> StorageInteractResult<()>;
}

pub trait StorageDir {
    /// If the parent_dir exist and its been given
    /// by the consuming OS we can then make the
    /// directory with the given path. This only has
    /// to run once, however it calls recursively the create_dir_all.
    /// Once the child path has been set it gets inserted
    /// into the map for Storage for constant time lookup and easy storage.
    fn make_dir_for_child(&self, path: impl AsRef<Path>) -> Result<PathBuf>;

    /// If the parent_dir exist and its been given
    /// by the consuming OS we can then delete the
    /// directory with the given path. This only has
    /// to run once, however it calls recursively the remove_dir_all.
    /// Once the child path has been deleted it gets removed
    /// from the map.
    fn remove_dir_for_child(&self, path: impl AsRef<Path>) -> Result<()>;

    /// This will generate a default file but will be reused with replacing data.
    /// In the module we can read and mutate content but then the
    /// file needs to be recreated. This does not hurt performance
    /// or cause any poor behavior. All file is always up to date and no file or data is lingering.
    fn touch_file_for_child(&self, child_dir: impl AsRef<Path>, bytes: Option<&str>) -> Result<()>;

    /// Read the contents of the files stored in the storage directory by a
    /// given path. Retrieve as a io buffer stream and return a
    /// string construct from it for easy parsing of data to structs or T types.
    fn stream_buffer_from_child(&self, path: impl AsRef<Path>) -> Result<String>;
}

pub type StorageDataValue = serde_json::Value;
