use crate::http::request::Request;
use crate::http::response::Response;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static SHARED: Lazy<Arc<Mutex<HttpClient>>> =
    Lazy::new(|| Arc::new(Mutex::new(HttpClient::new())));

pub struct HttpClient {
    callback: fn(Request) -> Response,
}

impl HttpClient {
    const fn new() -> Self {
        Self {
            callback: default_callback,
        }
    }
    pub fn set_callback(&mut self, callback: fn(Request) -> Response) {
        self.callback = callback;
    }
    pub fn send_request(&self, request: Request) -> Response {
        (self.callback)(request)
    }
}

fn default_callback(_request: Request) -> Response {
    Response::default()
}
