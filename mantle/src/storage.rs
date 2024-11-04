mod interface;
mod json_storage;

pub use interface::{StorageDataValue, StorageDir, StorageInteract};
pub use json_storage::{
    Storage, StorageInteractError, StorageInteractResult, SELECTED_REGION_STORAGE_KEY,
    STORAGE_APP_DIR, STORAGE_LOGGING_DIR, STORAGE_USER_DIR, STORAGE_USER_SESSION_KEY,
    STORAGE_USER_USER_SESSION_KEY,
};
