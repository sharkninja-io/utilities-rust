use crate::list::MantleList;
use log::debug;
use mantle_utilities::http::{
    request::{Header, Request},
    response::Response,
};
use mantle_utilities::string::MantleString;
use std::ffi::{c_uchar, c_ushort};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct IosRequest {
    url: *const c_char,
    method: *const c_char,
    body: *const MantleList<u8>,
    timeout: c_uchar,
    headers: *const MantleList<IosHeader>,
}

#[repr(C)]
#[derive(Debug)]
pub struct IosHeader {
    key: *const c_char,
    value: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct IosResponse {
    content: *const MantleList<u8>,
    status_code: c_ushort,
}

impl IosRequest {
    pub fn new_c_object(rust_object: &Request) -> Self {
        Self {
            url: MantleString(rust_object.url.to_owned()).to_ptr(),
            method: MantleString(rust_object.method.to_string()).to_ptr(),
            body: match &rust_object.body {
                Some(value) => MantleList::vec_to_list_ptr(value.to_vec()),
                None => std::ptr::null(),
            },
            timeout: rust_object.timeout.to_owned(),
            headers: {
                let array = MantleList::<IosHeader>::from(
                    rust_object
                        .headers
                        .iter()
                        .map(IosHeader::new_c_object)
                        .collect::<Vec<_>>(),
                );
                let boxed = Box::new(array);
                Box::into_raw(boxed)
            },
        }
    }
}

impl IosHeader {
    pub fn new_c_object(rust_object: &Header) -> Self {
        Self {
            key: MantleString(rust_object.key.to_owned()).to_ptr(),
            value: MantleString(rust_object.value.to_string()).to_ptr(),
        }
    }
}

impl IosResponse {
    /// # Safety
    ///
    /// `c_object_ptr` - must point to valid data or be null.
    pub unsafe fn new_rust_object(c_object_ptr: *const Self) -> Option<Response> {
        if c_object_ptr.is_null() {
            debug!("Response pointer was null");
            None
        } else {
            let c_response = &*c_object_ptr;
            let content = MantleList::copy_to_vec_ptr(c_response.content);
            let status_code = c_response.status_code.to_owned().try_into().unwrap();
            let response = Response {
                headers: Default::default(),
                content,
                status_code,
            };
            Some(response)
        }
    }
}

impl Default for IosResponse {
    fn default() -> Self {
        Self {
            content: std::ptr::null(),
            status_code: 0,
        }
    }
}
