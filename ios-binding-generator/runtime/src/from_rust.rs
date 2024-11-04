use ios_utilities::list::MantleList;
use ios_utilities::result::MantleResult;
use mantle_utilities::error::MantleResultError;
use mantle_utilities::string::MantleString;
use serde::Serialize;
use std::ffi::c_char;

pub fn string_to_string_ptr(string: String) -> *const c_char {
    MantleString(string).to_ptr()
}

pub fn vec_to_primitive_list<T: Copy>(vec: Vec<T>) -> *const MantleList<T> {
    Box::into_raw(Box::new(MantleList::from(vec)))
}

pub fn primitive_result_to_mantle_result<T: Copy>(
    result: Result<T, Box<dyn MantleResultError>>,
) -> MantleResult<T> {
    match result {
        Ok(r) => MantleResult::new_success(r),
        Err(err) => MantleResult::new_fail(err.as_ref()),
    }
}

pub fn string_result_to_mantle_result(
    result: Result<String, Box<dyn MantleResultError>>,
) -> MantleResult<*const c_char> {
    match result {
        Ok(string) => MantleResult::new_success(string_to_string_ptr(string)),
        Err(err) => MantleResult::new_fail(err.as_ref()),
    }
}

pub fn list_result_to_mantle_result<T: Copy>(
    result: Result<Vec<T>, Box<dyn MantleResultError>>,
) -> MantleResult<MantleList<T>> {
    match result {
        Ok(vec) => MantleResult::new_success(MantleList::from(vec)),
        Err(err) => MantleResult::new_fail(err.as_ref()),
    }
}

pub fn serialized_result_to_mantle_result<T: Serialize>(
    result: Result<T, Box<dyn MantleResultError>>,
) -> MantleResult<MantleList<u8>> {
    match result.and_then(|vec| {
        serde_json::to_vec(&vec).map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
    }) {
        Ok(vec) => MantleResult::new_success(MantleList::from(vec)),
        Err(err) => MantleResult::new_fail(err.as_ref()),
    }
}

pub fn void_result_to_mantle_result(
    result: Result<(), Box<dyn MantleResultError>>,
) -> MantleResult<()> {
    match result {
        Ok(_) => MantleResult::new_success(()),
        Err(err) => MantleResult::new_fail(err.as_ref()),
    }
}

pub fn err_to_mantle_result<T, E: MantleResultError + ?Sized>(err: &E) -> MantleResult<T> {
    MantleResult::new_fail(err)
}
