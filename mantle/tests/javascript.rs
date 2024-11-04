#[cfg(feature = "http-impl")]
use mantle_utilities::javascript::script_download_file::update_script_file_version;
use serde_json::json;
use httpmock::{MockServer, When};
use js_sandbox::{Script, AnyError};

#[cfg(feature = "http-impl")]
#[test]
fn get_file_from_server() {
    let server = MockServer::start();
    let file_mock = server.mock(|when, then| {
        when.method(httpmock::prelude::GET)
            .path(format!(
                "/apiv1/javascript/script-0.3.2.js"
            ));
        then.status(200).json_body(json!({}));
    });

    let response = update_script_file_version(&server.url("/apiv1/javascript/script-0.3.2.js"));

    assert!(response.status().is_success());
}

/*
fn js_func_test_no_return() -> Result<(), AnyError> {
    let script_file_path = Path::new("./src/javascript/script-0.3.2.js");
    let js_code = fs::read_to_string(script_file_path).expect("Unable to read file");
	let mut script = Script::from_string(&js_code)?;

    let arg = "";
	let _result: String = script.call("greetings", (arg,))?;

    Ok(())
}

fn js_func_test_with_return() -> Result<i32, AnyError> {
    let script_file_path = Path::new("./src/javascript/script-0.3.2.js");
    let js_code = fs::read_to_string(script_file_path).expect("Unable to read file");
	let mut script = Script::from_string(&js_code)?;

	let arg = 7;
	let result: i32 = script.call("triple", (arg,))?;

	Ok(result)
}
*/