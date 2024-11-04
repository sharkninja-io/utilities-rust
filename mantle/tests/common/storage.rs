use crate::common::TestDir;
use mantle_utilities::storage::Storage;
use std::path::{Path, PathBuf};

// Storage with auto-cleanup on drop.
#[derive(Debug)]
pub struct TestStorage {
    storage: Storage,
    test_dir: TestDir,
}

impl TestStorage {
    pub fn new() -> Self {
        Self::with_path(Self::gen_storage_path())
    }

    pub fn with_path(path: impl AsRef<Path>) -> Self {
        let test_dir = TestDir::with_path(path);
        TestStorage {
            storage: Storage::new(test_dir.as_ref()).unwrap(),
            test_dir,
        }
    }

    // TestStorage shouldn't be dropped until the returned storage exist.
    pub fn storage(&self) -> Storage {
        self.storage.clone()
    }

    pub fn child_exists(&self, child: impl AsRef<Path>) -> bool {
        self.storage.parent_path().join(child).try_exists().unwrap()
    }

    pub fn gen_storage_path() -> PathBuf {
        TestDir::create_tmp_dir()
    }
}

impl AsRef<Storage> for TestStorage {
    fn as_ref(&self) -> &Storage {
        &self.storage
    }
}
