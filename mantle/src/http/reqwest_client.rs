use std::error::Error;
use std::io::Read;

#[cfg(feature = "http-impl")]
use reqwest::blocking::RequestBuilder;

use Method::{DELETE, POST, PUT};

use crate::http::client::SHARED;
use crate::http::request::{Method, Request};
use crate::http::response::{Response, StatusCode};

pub struct ReqwestClient {}

impl ReqwestClient {
    fn create_request(request: Request) -> Result<RequestBuilder, Box<dyn Error>> {
        // NOTE: Re-use this if in a prod scenario
        let client = reqwest::blocking::Client::new();
        // NOTE: Re-use this if in a prod scenario
        let mut builder = match request.method {
            POST => client.post(request.url),
            PUT => client.put(request.url),
            DELETE => client.delete(request.url),
            _ => client.get(request.url),
        };
        for header in request.headers.iter() {
            builder = builder.header(header.key.clone(), header.value.clone());
        }
        if let Some(body) = request.body {
            builder = builder.body(String::from_utf8(body)?);
        }
        Ok(builder)
    }

    pub fn send_request(request: Request) -> Response {
        let Ok(reqwest_request) = ReqwestClient::create_request(request) else {
            return Response::default();
        };
        let Ok(mut http_response) = reqwest_request.send() else {
            return Response::default();
        };
        let Ok(status_code) = StatusCode::try_from(http_response.status().as_u16()) else {
            return Response::default();
        };

        let mut content = vec![];
        let _ = http_response.read_to_end(&mut content);
        let headers = http_response
            .headers()
            .into_iter()
            .map(|(value, key)| (value.to_string(), key.as_bytes().to_vec()))
            .collect();
        Response {
            headers,
            status_code,
            content,
        }
    }

    // NOTE: This is created to execute requests for functions in examples ONLY.
    pub fn set_as_global_http_callback() {
        if let Ok(mut shared) = SHARED.clone().lock() {
            shared.set_callback(ReqwestClient::send_request);
            drop(shared);
        };
    }
}
