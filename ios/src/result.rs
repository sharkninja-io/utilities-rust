use crate::error::MantleErrorFFI;
use mantle_utilities::error::MantleResultError;

#[repr(C)]
#[derive(Debug)]
pub enum MantleResult<T> {
    Success(*const T),
    Fail(*const MantleErrorFFI),
}

impl<T> MantleResult<T> {
    pub fn new_success(success: T) -> Self {
        let boxed = Box::new(success);
        let success = Box::into_raw(boxed);
        MantleResult::Success(success)
    }

    pub fn new_fail<E: MantleResultError + ?Sized>(err: &E) -> Self {
        let mantle_err = MantleErrorFFI::new(err.error_type(), err.error_description());
        MantleResult::Fail(Box::into_raw(Box::new(mantle_err)))
    }
}

impl<T, E: MantleResultError + ?Sized> From<&E> for MantleResult<T> {
    fn from(err: &E) -> Self {
        MantleResult::new_fail(err)
    }
}
