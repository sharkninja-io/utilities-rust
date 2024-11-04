use fake::{Fake, Faker};
use mantle_utilities::logs::*;
use std::num::NonZeroUsize;
use tempfile::{tempdir, TempDir};

// Reuse the same dir and run the tests sequentially
// because we can set the log folder only once at the start of the program.
#[test]
fn test_log() {
    let temp_dir = tempdir().unwrap();
    writes_to_log(&temp_dir);
    appends_at_new_line(&temp_dir);
}

fn writes_to_log(temp_dir: &TempDir) {
    let test_log_file_1 = "test_log_file_1.log";
    let test_log_file_2 = "test_log_file_2.log";
    let value: String = Faker.fake();
    set_log_dir(temp_dir.path(), NonZeroUsize::new(10).unwrap()).unwrap();

    // Write to log file
    log(test_log_file_1, &value).unwrap();

    // Read from log file
    let result = get_log(test_log_file_1).unwrap();
    assert_eq!(result.trim_end(), value);

    // Write to log file
    log(test_log_file_2, &value).unwrap();

    // Read from log file
    let result = get_log(test_log_file_2).unwrap();
    assert_eq!(result.trim_end(), value);
}

fn appends_at_new_line(temp_dir: &TempDir) {
    let test_log_file_2 = "test_log_file_3.log";
    let value: String = Faker.fake();

    set_log_dir(temp_dir.path(), NonZeroUsize::new(10).unwrap()).unwrap();

    log(test_log_file_2, &value).unwrap();
    log(test_log_file_2, &value).unwrap();
    log(test_log_file_2, &value).unwrap();
    let result = get_log(test_log_file_2).unwrap();

    assert_eq!(result, format!("{value}\n{value}\n{value}\n"));
}
