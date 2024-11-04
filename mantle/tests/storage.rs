use crate::common::storage::TestStorage;
use fake::{Fake, Faker};
use mantle_utilities::storage::{StorageDataValue, StorageDir, StorageInteract};
use std::ffi::OsString;
use std::path::Path;
use std::sync::{Arc, Barrier};
use std::{fs, thread};

mod common;

const APP_DIR: &str = "app";
const USER_DIR: &str = "user";
const TOP_LEVEL_DIR: &str = "cloudcore";
const LOGGING_DIR: &str = "logging";
const STORE_FILE: &str = ".store";
const DEFAULT_BYTES_IN_FILE: &str = r#"{"data":{}}"#;
const REGION_KEY: &str = "countryRegionSelection";

#[test]
fn storage_dir_has_correct_layout() {
    let test_storage = TestStorage::new();
    let path = test_storage.as_ref().parent_path();
    let mut entries: Vec<OsString> = path
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    entries.sort();

    assert_eq!(TOP_LEVEL_DIR, path.file_name().unwrap());
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], APP_DIR);
    assert_eq!(entries[1], USER_DIR);
    assert!(has_only_one_entry(path.join(APP_DIR), STORE_FILE));
    assert!(has_only_one_entry(path.join(USER_DIR), STORE_FILE));

    let app_content = fs::read_to_string(path.join(APP_DIR).join(STORE_FILE)).unwrap();
    let user_content = fs::read_to_string(path.join(USER_DIR).join(STORE_FILE)).unwrap();
    assert_eq!(DEFAULT_BYTES_IN_FILE, app_content);
    assert_eq!(DEFAULT_BYTES_IN_FILE, user_content);
}

#[test]
fn creates_db_dir() {
    let test_storage = TestStorage::new();
    let path = test_storage.as_ref().parent_path();
    let parent = path.parent().unwrap();
    assert!(parent.join("store").try_exists().unwrap());
}

#[test]
fn existing_logs_are_removed() {
    let storage_path = TestStorage::gen_storage_path();
    let logging_dir = storage_path.join(TOP_LEVEL_DIR).join(LOGGING_DIR);
    fs::create_dir_all(&logging_dir).unwrap();
    gen_file_with_random_content(&logging_dir);

    let _ = TestStorage::with_path(storage_path);
    assert!(!logging_dir.try_exists().unwrap());
}

#[test]
fn storage_dir_is_empty_after_wipe() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let storage_path = storage.parent_path();
    let dir_path = storage_path.join(Faker.fake::<String>());
    fs::create_dir(&dir_path).unwrap();
    gen_file_with_random_content(dir_path);

    storage.wipe_storage(true, true).unwrap();

    assert!(is_empty_dir(storage_path));
}

#[test]
fn wipe_with_flag_does_not_remove_app_dir() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();

    storage.wipe_storage(false, true).unwrap();
    storage.wipe_storage(false, false).unwrap();

    let app_path = storage.parent_path().join(APP_DIR);
    assert!(app_path.try_exists().unwrap());
}

#[test]
fn wipe_with_flag_retains_selected_region() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    storage.set_value(USER_DIR, REGION_KEY, "UK").unwrap();

    storage.wipe_storage(true, false).unwrap();
    storage.wipe_storage(false, false).unwrap();

    let region = storage.get_value(USER_DIR, REGION_KEY).unwrap();
    assert!(matches!(region, StorageDataValue::String(string) if string == "UK"));
}

#[test]
fn make_dir_for_child_creates_dir() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();

    let full_path = storage.make_dir_for_child(&child_dir).unwrap();
    assert_eq!(full_path, storage.parent_path().join(&child_dir));
    assert_eq!(
        storage.stream_buffer_from_child(child_dir).unwrap(),
        DEFAULT_BYTES_IN_FILE
    );
}

#[test]
fn remove_dir_for_child_removes_dir() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let full_path = storage.make_dir_for_child(&child_dir).unwrap();

    storage.remove_dir_for_child(child_dir).unwrap();

    assert!(!full_path.try_exists().unwrap());
}

#[test]
fn touch_file_for_child_writes_with_default_bytes() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();

    storage.touch_file_for_child(&child_dir, None).unwrap();

    let file_content =
        fs::read_to_string(storage.parent_path().join(child_dir).join(STORE_FILE)).unwrap();
    assert_eq!(file_content, DEFAULT_BYTES_IN_FILE);
}

#[test]
fn touch_file_for_child_writes_with_custom_bytes() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let bytes: String = Faker.fake();

    storage
        .touch_file_for_child(&child_dir, Some(bytes.as_ref()))
        .unwrap();

    let file_content =
        fs::read_to_string(storage.parent_path().join(child_dir).join(STORE_FILE)).unwrap();
    assert_eq!(file_content, bytes);
}

#[test]
fn stream_buffer_from_child_reads_content() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let bytes: String = Faker.fake();

    storage
        .touch_file_for_child(&child_dir, Some(bytes.as_ref()))
        .unwrap();
    let content = storage.stream_buffer_from_child(child_dir).unwrap();

    assert_eq!(content, bytes);
}

#[test]
fn sets_and_gets_null_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let null_key: String = Faker.fake();
    let null_value: Option<()> = None;

    storage
        .set_value(&child_dir, &null_key, null_value)
        .unwrap();
    let got_value = storage.get_value(&child_dir, &null_key).unwrap();

    assert!(matches!(got_value, StorageDataValue::Null));
}

#[test]
fn sets_and_gets_bool_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let bool_key: String = Faker.fake();
    let bool_value: bool = Faker.fake();

    storage
        .set_value(&child_dir, &bool_key, bool_value)
        .unwrap();
    let got_value = storage.get_value(&child_dir, &bool_key).unwrap();

    assert!(matches!(got_value, StorageDataValue::Bool(bool) if bool == bool_value));
}

#[test]
fn sets_and_gets_number_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let number_key: String = Faker.fake();
    let number_value: i32 = Faker.fake();

    storage
        .set_value(&child_dir, &number_key, number_value)
        .unwrap();
    let got_value = storage.get_value(&child_dir, &number_key).unwrap();

    assert!(matches!(
        got_value,
        StorageDataValue::Number(number) if number.as_i64().unwrap() as i32 == number_value
    ));
}

#[test]
fn sets_and_gets_double_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let double_key: String = Faker.fake();
    // Should always has the fractional part. Otherwise will be deserialized as integer.
    let double_value: f64 = Faker.fake::<u32>() as f64 + 0.5;

    storage
        .set_value(&child_dir, &double_key, double_value)
        .unwrap();
    let got_value = storage.get_value(&child_dir, &double_key).unwrap();

    assert!(matches!(
        got_value,
        StorageDataValue::Number(double) if double.as_f64().unwrap() == double_value
    ));
}

#[test]
fn sets_and_gets_string_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let string_key: String = Faker.fake();
    let string_value: String = Faker.fake();

    storage
        .set_value(&child_dir, &string_key, &string_value)
        .unwrap();
    let got_value = storage.get_value(&child_dir, &string_key).unwrap();

    assert!(matches!(got_value, StorageDataValue::String(string) if string == string_value));
}

#[test]
fn sets_and_gets_object_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let object_key: String = Faker.fake();
    let object_value: Vec<u32> = Faker.fake();

    storage
        .set_value(&child_dir, &object_key, &object_value)
        .unwrap();

    if let StorageDataValue::Array(obj) = storage.get_value(&child_dir, &object_key).unwrap() {
        let got_array: Vec<u32> = obj
            .into_iter()
            .map(|value| value.as_u64().unwrap() as u32)
            .collect();

        assert_eq!(got_array, object_value);
    } else {
        panic!("not an object");
    };
}

#[test]
fn removes_value() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let child_dir: String = Faker.fake();
    let key: String = Faker.fake();
    let value: i32 = Faker.fake();
    storage.set_value(&child_dir, &key, value).unwrap();

    storage.remove_value(&child_dir, &key).unwrap();

    assert!(matches!(
        storage.get_value(child_dir, &key),
        Ok(StorageDataValue::Null)
    ));
}

#[test]
fn inits_device_storage_by_dsn() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let dsn: String = Faker.fake();

    storage.init_device_storage(&dsn).unwrap();

    assert!(test_storage.child_exists(dsn));
}

#[test]
fn clears_device_storage_by_dsn() {
    let test_storage = TestStorage::new();
    let storage = test_storage.as_ref();
    let dsn: String = Faker.fake();
    storage.init_device_storage(&dsn).unwrap();

    storage.clear_device_storage(&dsn).unwrap();

    assert!(!test_storage.child_exists(dsn));
}

// Multithreading tests.
#[test]
fn returns_the_same_data_from_multiple_threads() {
    const NUMBER_OF_THREADS: usize = 4;
    let test_storage = TestStorage::new();
    let child_dir: String = Faker.fake();
    let key: String = Faker.fake();
    let value: String = Faker.fake();
    let barrier = Arc::new(Barrier::new(NUMBER_OF_THREADS));
    test_storage
        .as_ref()
        .set_value(&child_dir, &key, &value)
        .unwrap();

    thread::scope(|scope| {
        for _ in 0..NUMBER_OF_THREADS {
            let storage = test_storage.storage();
            let child_dir = child_dir.clone();
            let key = key.clone();
            let value = value.clone();
            let barrier = Arc::clone(&barrier);
            scope.spawn(move || {
                barrier.wait();
                let got = storage.get_value(&child_dir, &key).unwrap();
                assert!(matches!(got, StorageDataValue::String(string) if string == value))
            });
        }
    });
}

#[test]
fn storage_does_not_have_data_races() {
    const NUMBER_OF_THREADS: usize = 128;
    let test_storage = TestStorage::new();
    let child_dir: String = Faker.fake();
    let key: String = Faker.fake();
    let mut possible_values: [i64; NUMBER_OF_THREADS] = [0; NUMBER_OF_THREADS];
    for value in possible_values.iter_mut() {
        *value = Faker.fake();
    }
    let barrier = Barrier::new(NUMBER_OF_THREADS);

    thread::scope(|scope| {
        for value in &possible_values {
            scope.spawn(|| {
                let storage = test_storage.storage();
                barrier.wait();
                storage.set_value(&child_dir, &key, *value).unwrap();
                let got_value = storage.get_value(&child_dir, &key).unwrap();
                assert!(matches!(
                    got_value,
                    StorageDataValue::Number(int) if possible_values.contains(&int.as_i64().unwrap())
                ));
            });
        }
    });
}

fn has_only_one_entry(path: impl AsRef<Path>, entry_name: &str) -> bool {
    let mut iter = path.as_ref().read_dir().unwrap();
    let equal_name = if let Some(entry) = iter.next() {
        entry.unwrap().file_name() == entry_name
    } else {
        false
    };
    equal_name && iter.next().is_none()
}

fn is_empty_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().read_dir().unwrap().next().is_none()
}

fn gen_file_with_random_content(path_to_dir: impl AsRef<Path>) {
    fs::write(
        path_to_dir.as_ref().join(Faker.fake::<String>()),
        Faker.fake::<String>(),
    )
    .unwrap();
}
