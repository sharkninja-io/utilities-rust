use ios_utilities::list::MantleList;
use mantle_utilities::string::MantleStringPointer;
use serde::de::DeserializeOwned;
use std::ffi::c_char;

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn string_ptr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    MantleStringPointer(ptr).to_string()
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn mantle_primitive_list_to_vec<T: Copy>(list: *const MantleList<T>) -> Vec<T> {
    list.as_ref()
        .map(|list| list.copy_to_vec())
        .unwrap_or_default()
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn mantle_string_list_to_vec(list: *const MantleList<*const c_char>) -> Vec<String> {
    if let Some(list) = list.as_ref() {
        list.as_slice()
            .iter()
            .map(|&str_ptr| string_ptr_to_string(str_ptr))
            .collect()
    } else {
        Vec::new()
    }
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn from_mantle_bytes_to_serialized<T: DeserializeOwned>(
    json: *const MantleList<u8>,
) -> serde_json::Result<T> {
    let bytes = json.as_ref().map(|list| list.as_slice()).unwrap_or(&[]);

    serde_json::from_slice(bytes)
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn ptr_to_optional_primitive<T: Copy>(primitive: *const T) -> Option<T> {
    primitive.as_ref().cloned()
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn string_ptr_to_optional_string(ptr: *const c_char) -> Option<String> {
    MantleStringPointer(ptr).to_option_string()
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn mantle_primitive_list_to_optional_vec<T: Copy>(
    list: *const MantleList<T>,
) -> Option<Vec<T>> {
    list.as_ref().map(|list| list.copy_to_vec())
}

/// # Safety
///
/// ffi function, by default can only be used in unsafe
pub unsafe fn from_mantle_bytes_to_optional_serialized<T: DeserializeOwned>(
    json: *const MantleList<u8>,
) -> serde_json::Result<Option<T>> {
    if json.is_null() {
        return Ok(None);
    }

    from_mantle_bytes_to_serialized(json).map(|r| Some(r))
}
