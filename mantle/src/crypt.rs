use std::fs;
use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCryptError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct EncrKey{
    key: String
}

fn get_key(key_path: &str) -> String {
    let file = fs::File::open(key_path).unwrap();
    let yaml: EncrKey = serde_yaml::from_reader(file).unwrap();
    yaml.key
}

#[test]
fn test_get_key() {
    assert_eq!("auAywYOPGzKH5gcuA467dwpn6tId79IW", get_key("config_encr.yml"));
}

pub fn encrypt_str_key_from_file(str: &str, key_path: &str) -> String {
    let mc = new_magic_crypt!(get_key(key_path));
    mc.encrypt_str_to_base64(str)
}

pub fn decrypt_str_key_from_file(str: &str, key_path: &str) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(get_key(key_path));
    mc.decrypt_base64_to_string(str)
}
pub fn encrypt_str(str: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key);
    mc.encrypt_str_to_base64(str)
}

pub fn decrypt_str(str: &str, key: &str) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(key);
    mc.decrypt_base64_to_string(str)
}

#[test]
fn magic_crypt_test() {
    use fake::Fake;
    let str: String = fake::Faker.fake();
    let key_path = "config_encr.yml";
    let base64 = encrypt_str_key_from_file(&str, key_path.clone());
    let result = decrypt_str_key_from_file(&base64, key_path).unwrap();
    assert_eq!(str, result);
}