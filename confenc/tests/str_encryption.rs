use confenc::confenc;

#[test]
fn encrypt_config_value_str_should_be_equal_to_the_original() {
    assert_eq!("value", confenc!("confenc/tests/config.yml", "key"));
    assert_eq!(
        "nested value",
        confenc!("confenc/tests/config.yml", "nested.key")
    );
}
