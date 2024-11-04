#[cfg(feature = "http-impl")]
use reqwest;

#[cfg(feature = "http-impl")]
pub fn update_script_file_version(url: &str) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();

    let response = client.get( url ).send().unwrap();
    response

    //GET file from the server

    //COMPARE files versions

    //DO NOTHING if versions identical

    //REPLACE script file if server version is newer
}