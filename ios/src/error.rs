use std::ffi::c_char;

use mantle_utilities::string::MantleString;

#[repr(C)]
#[derive(Debug)]
pub struct MantleErrorFFI {
    pub error_type: *const c_char,
    pub description: *const c_char,
}

impl MantleErrorFFI {
    pub fn new(error_type: String, description: String) -> MantleErrorFFI {
        MantleErrorFFI {
            error_type: MantleString(error_type).to_ptr(),
            description: MantleString(description).to_ptr(),
        }
    }
}
