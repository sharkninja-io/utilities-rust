pub mod java_class_names;
pub mod java_signatures;
pub mod jni_exts;
pub mod mantle_error;
#[cfg(feature = "mqtt")]
pub mod mqtt;
pub mod result;

#[cfg(feature = "http")]
pub mod http;

mod callback;
mod traits;

pub use callback::holder::CallbackStruct;
pub use callback::{invoke_callback, invoke_callback_object, invoke_result_callback};
pub use result::{error_result, success_result, AndroidResult};

const JAVA_PACKAGE: &str = "com/sharkninja/api/mantleutilities/";
