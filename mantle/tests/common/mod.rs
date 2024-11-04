#![allow(dead_code)]

use std::path::{Path, PathBuf};
use uuid::Uuid;

pub mod storage;

// Temp dir with auto-cleanup on drop.
#[derive(Debug)]
pub struct TestDir {
    path: PathBuf,
}

impl TestDir {
    pub fn new() -> Self {
        TestDir {
            path: Self::create_tmp_dir(),
        }
    }

    pub fn with_path(path: impl AsRef<Path>) -> Self {
        TestDir {
            path: path.as_ref().to_path_buf(),
        }
    }

    fn create_tmp_dir() -> PathBuf {
        // Test isolation
        let test_dir_name = Uuid::new_v4().to_string();
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push(test_dir_name);
        std::fs::create_dir(&tmp_dir).unwrap();
        tmp_dir
    }
}

impl AsRef<Path> for TestDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}
