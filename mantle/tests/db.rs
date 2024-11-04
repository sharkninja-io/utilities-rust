use crate::common::TestDir;
use fake::{Dummy, Fake, Faker};
use mantle_utilities::{db::sled_db::SledDb, javascript::javascript::JavaScriptFile};
use mantle_utilities::db::Db;
use serde::{Deserialize, Serialize};
use std::thread;

mod common;

type TestKey = String;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Dummy)]
struct TestValue {
    number: u32,
    string: String,
    vector: Vec<String>,
    optional: Option<Vec<u8>>,
    boolean: bool,
}

#[test]
fn bucket_stores_data() {
    let db_dir = TestDir::new();
    let bucket_id: String = Faker.fake();
    let first_test_key: TestKey = Faker.fake();
    let second_test_key: TestKey = Faker.fake();
    let first_test_value: TestValue = Faker.fake();
    let second_test_value: TestValue = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(bucket_id).unwrap();

    bucket.insert(&first_test_key, &first_test_value).unwrap();
    bucket.insert(&second_test_key, &second_test_value).unwrap();

    let got_first_value = bucket.get(&first_test_key).unwrap().unwrap();
    let got_second_value = bucket.get(&second_test_key).unwrap().unwrap();

    assert_eq!(got_first_value, first_test_value);
    assert_eq!(got_second_value, second_test_value);
}

#[test]
fn bucket_returns_none_without_data() {
    let db_dir = TestDir::new();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(Faker.fake::<String>()).unwrap();

    let got: Option<TestValue> = bucket.get(&Faker.fake::<TestKey>()).unwrap();

    assert!(got.is_none());
}

#[test]
fn data_retains_after_db_close() {
    let db_dir = TestDir::new();
    let test_key: TestKey = Faker.fake();
    let test_value: TestValue = Faker.fake();
    let bucket_id: String = Faker.fake();

    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(&bucket_id).unwrap();
    bucket.insert(&test_key, &test_value).unwrap();
    drop(db);
    drop(bucket);

    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(&bucket_id).unwrap();
    let got_value = bucket.get(&test_key).unwrap();

    assert_eq!(got_value, Some(test_value));
}

#[test]
fn can_open_multiple_independent_buckets() {
    let db_dir = TestDir::new();
    let mut bucket_id: String = Faker.fake();
    let test_key: TestKey = Faker.fake();
    let first_test_value: TestValue = Faker.fake();
    let second_test_value: TestValue = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let first_bucket = db.open_bucket(&bucket_id).unwrap();
    bucket_id.push(Faker.fake());
    let second_bucket = db.open_bucket(bucket_id).unwrap();

    first_bucket.insert(&test_key, &first_test_value).unwrap();
    second_bucket.insert(&test_key, &second_test_value).unwrap();

    let got_first_value = first_bucket.get(&test_key).unwrap();
    let got_second_value = second_bucket.get(&test_key).unwrap();

    assert_eq!(got_first_value, Some(first_test_value));
    assert_eq!(got_second_value, Some(second_test_value));
}

#[test]
fn deletes_bucket() {
    let db_dir = TestDir::new();
    let bucket_id: String = Faker.fake();
    let test_key: TestKey = Faker.fake();
    let test_value: TestValue = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(&bucket_id).unwrap();
    bucket.insert(&test_key, &test_value).unwrap();
    drop(bucket);

    db.delete_bucket(&bucket_id).unwrap();
    let bucket = db.open_bucket(&bucket_id).unwrap();
    let got_value: Option<TestValue> = bucket.get(&test_key).unwrap();

    assert!(got_value.is_none());
}

#[test]
fn removes_item() {
    let db_dir = TestDir::new();
    let test_key: TestKey = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(Faker.fake::<String>()).unwrap();

    bucket
        .insert(&test_key, &Faker.fake::<TestValue>())
        .unwrap();
    bucket.remove(&test_key).unwrap();
    let got_value = bucket.get(&test_key).unwrap();

    assert!(got_value.is_none());
}

#[test]
fn clears_bucket() {
    let db_dir = TestDir::new();
    let test_key: TestKey = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(Faker.fake::<String>()).unwrap();

    bucket
        .insert(&test_key, &Faker.fake::<TestValue>())
        .unwrap();
    bucket.clear().unwrap();
    let got_value = bucket.get(&test_key).unwrap();

    assert!(got_value.is_none());
}

#[test]
fn iterates() {
    let db_dir = TestDir::new();
    let mut data: Vec<(TestKey, TestValue)> =
        std::iter::repeat_with(|| Faker.fake()).take(100).collect();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let bucket = db.open_bucket(&Faker.fake::<String>()).unwrap();
    for item in data.iter() {
        bucket.insert(&item.0, &item.1).unwrap();
    }

    for item in bucket.iter().unwrap().map(|res| res.unwrap()) {
        let idx = data.iter().position(|i| i == &item).unwrap();
        data.remove(idx);
    }

    assert!(data.is_empty());
}

#[test]
#[allow(clippy::redundant_clone)]
fn can_be_used_from_multiple_threads() {
    let db_dir = TestDir::new();
    let bucket_id: String = Faker.fake();
    let (first_key, first_value): (TestKey, TestValue) = Faker.fake();
    let (second_key, second_value): (TestKey, TestValue) = Faker.fake();
    let db = Db::new(Box::new(SledDb::open(&db_dir).unwrap()));
    let cloned_db = db.clone();
    let bucket = db.open_bucket(&bucket_id).unwrap();
    let cloned_bucket = bucket.clone();
    thread::scope(|scope| {
        scope.spawn(|| {
            let bucket = cloned_db.open_bucket(&bucket_id).unwrap();
            bucket.insert(&first_key, &first_value).unwrap();
        });
        scope.spawn(|| {
            cloned_bucket.insert(&second_key, &second_value).unwrap();
        });
    });

    assert_eq!(Some(first_value), bucket.get(&first_key).unwrap());
    assert_eq!(Some(second_value), bucket.get(&second_key).unwrap());
}

#[cfg(feature = "http-impl")]
#[test]
fn javascript_file_deps_download() {
    use mantle_utilities::http::reqwest_client::ReqwestClient;
    
    ReqwestClient::set_as_global_http_callback();
    let base_path = "./src".to_string();
    let db_engine_implementation = SledDb::open("javascript").unwrap();
    let download_url = "https://sn-iot-app-dev.s3.amazonaws.com/shark/";
    let manifest_name = "manifest";
    let db = Db::new(Box::new(db_engine_implementation));
    let bucket = db.open_bucket("files").unwrap();
    JavaScriptFile::update_db_from_manifest(&bucket, base_path, download_url, manifest_name);
    
    dbg!(bucket);
}