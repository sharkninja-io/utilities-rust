use lru::LruCache;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();
static LOG_CACHE: OnceLock<Mutex<LruCache<String, File>>> = OnceLock::new();

/// Set the log directory. cache_size is the maximum size of cache for log files.
pub fn set_log_dir(
    log_dir: impl AsRef<Path>,
    cache_size: NonZeroUsize,
) -> std::result::Result<(), Error> {
    let log_dir = log_dir.as_ref();
    delete_logs(log_dir)?;

    LOG_CACHE.get_or_init(|| Mutex::new(LruCache::new(cache_size)));
    LOG_DIR.get_or_init(|| log_dir.to_path_buf());
    Ok(())
}
/// Deletes all files in the logs directory
pub fn delete_logs(log_dir: impl AsRef<Path>) -> std::result::Result<(), Error> {
    let log_dir = log_dir.as_ref();

    // Check if the given path exists and is a directory
    if !log_dir.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Path is not a directory"));
    }

    // Reading the contents of a directory
    let entries = std::fs::read_dir(log_dir)?;

    // Go through all the files in the directory and delete them
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }

    Ok(())
}
/// Write the logs to a file, if the file does not exist, a new file is created.
pub fn log(file_name: impl AsRef<str>, log: impl AsRef<str>) -> Result<()> {
    let log_dir = match LOG_DIR.get() {
        Some(dir) => dir,
        None => return Err(Error::new(ErrorKind::NotFound, "Log directory not set")),
    };

    // Check if the file is in the cache
    if let Some(file) = LOG_CACHE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .get_mut(file_name.as_ref())
    {
        // Write log to file
        writeln!(file, "{}", log.as_ref())?;
        return Ok(());
    }

    // If the file is not in the cache, create a new one and add it to the cache
    let path = log_dir.join(file_name.as_ref());
    let mut file = File::create(path)?;
    writeln!(file, "{}", log.as_ref())?;
    LOG_CACHE
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .put(file_name.as_ref().to_string(), file);

    Ok(())
}

/// Get all the logs in the file
pub fn get_log(file_name: impl AsRef<str>) -> Result<String> {
    let log_dir = match LOG_DIR.get() {
        Some(dir) => dir,
        None => return Err(Error::new(ErrorKind::NotFound, "Log directory not set")),
    };

    let path = log_dir.join(file_name.as_ref());
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
