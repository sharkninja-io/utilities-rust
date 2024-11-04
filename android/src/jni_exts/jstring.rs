use jni::objects::JString;
use jni::JNIEnv;
use log::error;
use mantle_utilities::string::MantleStringPointer;
use std::os::raw::c_char;
use std::ptr;

pub struct MantleJString<'a>(pub JString<'a>);
impl<'a> MantleJString<'a> {
    pub fn to_char_ptr(self, jni_env: JNIEnv) -> *const c_char {
        if self.0.is_null() {
            return ptr::null();
        }
        jni_env.get_string_utf_chars(self.0).unwrap_or_else(|err| {
            error!("Error: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }

    /// # Safety
    /// `jni_env` must not be null
    pub unsafe fn to_string(self, jni_env: JNIEnv) -> String {
        MantleStringPointer(self.to_char_ptr(jni_env)).to_string()
    }

    /// # Safety
    /// `jni_env` must not be null
    pub unsafe fn to_string_option(self, jni_env: JNIEnv) -> Option<String> {
        (!self.0.is_null()).then(|| unsafe { self.to_string(jni_env) })
    }
}
