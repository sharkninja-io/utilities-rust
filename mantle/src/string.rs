use log::error;
use std::ffi::{c_char, CStr, CString};

pub struct MantleStringPointer(pub *const c_char);

pub struct MantleString(pub String);

impl MantleStringPointer {
    /// # Safety
    ///
    /// (Copied from the [`CStr`] docs)
    ///
    /// * The memory pointed to by `ptr` must contain a valid nul terminator at the
    ///   end of the string.
    ///
    /// * `ptr` must be [valid] for reads of bytes up to and including the null terminator.
    ///   This means in particular:
    ///
    ///     * The entire memory range of this `CStr` must be contained within a single allocated object!
    ///     * `ptr` must be non-null even for a zero-length cstr.
    ///
    /// * The memory referenced by the returned `CStr` must not be mutated for
    ///   the duration of lifetime `'a`.
    ///
    /// > **Note**: This operation is intended to be a 0-cost cast but it is
    /// > currently implemented with an up-front calculation of the length of
    /// > the string. This is not guaranteed to always be the case.
    #[allow(clippy::inherent_to_string)]
    pub unsafe fn to_string(self) -> String {
        CStr::from_ptr(self.0)
            .to_str()
            .unwrap_or_else(|err| {
                error!("Error converting *const c_char to String: {:?}", err);
                panic!();
            })
            .to_string()
    }

    /// # Safety
    ///
    /// The same as `to_string`. With one exception the pointer can be null.
    pub unsafe fn to_option_string(self) -> Option<String> {
        if self.0.is_null() {
            None
        } else {
            Some(self.to_string())
        }
    }
}

impl MantleString {
    pub fn to_ptr(self) -> *const c_char {
        CString::new(self.0)
            .unwrap_or_else(|err| {
                error!("Error converting String to *const c_char: {:?}", err);
                panic!();
            })
            .into_raw()
    }
}
