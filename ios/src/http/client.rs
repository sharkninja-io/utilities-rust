use super::request::{IosRequest, IosResponse};
use mantle_utilities::http::{client::SHARED, request::Request, response::Response};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

static REQUESTS_CB_STRUCT: Lazy<Arc<Mutex<RequestsCallback>>> =
    Lazy::new(|| Arc::new(Mutex::new(|_| &IosResponse::default())));

type RequestsCallback = fn(*const IosRequest) -> *const IosResponse;

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_set_requests_callback(requests_callback: RequestsCallback) {
    if let Ok(mut guard) = REQUESTS_CB_STRUCT.lock() {
        *guard = requests_callback;
        drop(guard);
    }
    if let Ok(mut shared) = SHARED.clone().lock() {
        shared.set_callback(handle_request);
        drop(shared);
    };
}

fn handle_request(request: Request) -> Response {
    let request_ptr = Box::into_raw(Box::new(IosRequest::new_c_object(&request)));
    let cbs = REQUESTS_CB_STRUCT.clone();
    if let Ok(fn_cb) = cbs.lock() {
        unsafe {
            let response_ptr = (fn_cb)(request_ptr);
            return IosResponse::new_rust_object(response_ptr).unwrap();
        }
    }
    Response::default()
}
