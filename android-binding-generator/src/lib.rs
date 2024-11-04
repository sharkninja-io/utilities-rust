//! This crate contains a macro for auto-gen android bindings.
//!
//! The macro can automatically de/serialize bytes in the JNI bindings using serde_json.
//!
//! The macro has the following syntax:
//!
//! ```text
//! impl_android_binding!(
//!     <result binding name>(
//!         JNIEnv, JClass(optional), <JNI type> = <Argument Conversion>
//!     ) -> <JNI return type> = <Output Conversion>,
//!     <a name of the rust function to call with converted args. The result of the function gets converted to the specified JNI return type>
//! );
//! ```
//!
//! The macro uses the [android_binding_runtime] crate.
//! Where you can check all the JNI to Rust and Rust to JNI code.
//!
//! JNIEnv **must** be the first arg to a binding. JClass **can** be the second arg.
//! These won't be passed to the rust function.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JClass;
//!# fn main() {
//! fn example() {}
//!
//! impl_android_binding!(with_jclass(JNIEnv, JClass), example);
//! impl_android_binding!(without_jclass(JNIEnv), example);
//!# }
//! ```
//!
//! # Supported Conversions
//!
//! ## Argument Conversion
//!
//! I64 - convert [jlong] to [i64]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jlong;
//!# fn main() {
//! fn example(number: i64) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jlong), example);
//!# }
//! ```
//!
//!
//!
//! I32 - convert [jint] to [i32]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jint;
//!# fn main() {
//! fn example(number: i32) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jint), example);
//!# }
//! ```
//!
//!
//! U32 - convert [jint] to [u32].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jint;
//!# fn main() {
//! fn example(number: u32) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jint = U32), example);
//!# }
//! ```
//!
//!
//! U8 - convert [jint] to [u8].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jint;
//!# fn main() {
//! fn example(number: u8) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jint = U8), example);
//!# }
//! ```
//!
//! Double - convert [jdouble] to [f64].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jdouble;
//!# fn main() {
//! fn example(double: f64) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jdouble), example);
//!# }
//! ```
//!
//!
//! Boolean - convert [jboolean] to [bool]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jboolean;
//!# fn main() {
//! fn example(boolean: bool) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, jboolean), example);
//!# }
//! ```
//!
//!
//! String - convert [JString] to [String]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JString;
//!# fn main() {
//! fn example(string: String) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JString), example);
//!# }
//! ```
//!
//!
//! Bytes - convert [JObject] (java array of bytes) to [Vec<u8>].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# fn main() {
//! fn example(bytes: Vec<u8>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = Bytes), example);
//!# }
//! ```
//!
//!
//! StringList - convert [JObject] (java array of strings) to [Vec<String>].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# fn main() {
//! fn example(strings: Vec<String>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = StringList), example);
//!# }
//! ```
//!
//!
//! Serialized(`SerdeType`) - convert [JObject] (java array of bytes) to any type that implements deserialization using serde_json.
//! Can be used in fallible functions only.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# use jni::sys::jobject;
//!# use serde::{Deserialize, Serialize};
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example(serde: ExampleStruct) -> Result<(), Box<dyn MantleResultError>> {
//!  unimplemented!()
//! }
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = Serialized(ExampleStruct)) -> jobject = FallibleVoid, example);
//!# }
//! ```
//!
//!
//! OptionalI32 - convert [JObject] (java Integer class) to [Option<i32>]. None if null.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# fn main() {
//! fn example(strings: Option<i32>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = OptionalI32), example);
//!# }
//! ```
//!
//!
//! OptionalU32 - convert [JObject] (java Integer class) to [Option<u32>]. None if null.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# fn main() {
//! fn example(strings: Option<u32>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = OptionalU32), example);
//!# }
//! ```
//!
//!
//! OptionalBool - convert [JObject] (java Boolean class) to [Option<bool>]. None if null.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# fn main() {
//! fn example(strings: Option<bool>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = OptionalBool), example);
//!# }
//! ```
//!
//!
//! OptionalString - convert [JString] to [Option<String>]. None if null.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JString;
//!# fn main() {
//! fn example(strings: Option<String>) {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JString = OptionalString), example);
//!# }
//! ```
//!
//!
//! OptionalSerialized(`SerdeType`) - convert [JObject] (java array of bytes) to any type that implements deserialization using serde_json.
//! None if the JObject is null. Can be used in fallible functions only.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::JObject;
//!# use jni::sys::jobject;
//!# use serde::{Deserialize, Serialize};
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example(serde: Option<ExampleStruct>) -> Result<(), Box<dyn MantleResultError>> {
//!  unimplemented!()
//! }
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! impl_android_binding!(example_binding(JNIEnv, JObject = OptionalSerialized(ExampleStruct)) -> jobject = FallibleVoid, example);
//!# }
//! ```
//!
//! ## Output conversion
//!
//! Fallible output conversions use [AndroidResult] from the android crate.
//!
//! I64 - convert [i64] to [jlong]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jlong;
//!# fn main() {
//! fn example() -> i64 { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jlong, example);
//!# }
//! ```
//!
//!
//!
//! I32 - convert [i32] to [jint]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jint;
//!# fn main() {
//! fn example() -> i32 { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jint, example);
//!# }
//! ```
//!
//!
//! Boolean - convert [bool] to [jboolean]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jboolean;
//!# fn main() {
//! fn example() -> bool { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jboolean, example);
//!# }
//! ```
//!
//!
//! String - convert [String] to [jstring]. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jstring;
//!# fn main() {
//! fn example() -> String { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jstring, example);
//!# }
//! ```
//!
//!
//! Bytes - convert [Vec<u8>] to [jobject] (java byte array).
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# fn main() {
//! fn example() -> Vec<u8> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = Bytes, example);
//!# }
//! ```
//!
//!
//! Void - don't return anything. You can omit this conversion.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jstring;
//!# fn main() {
//! fn example() { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv), example);
//!# }
//! ```
//!
//!
//! FallibleI64 - convert `Result<i64, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<i64, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleI64, example);
//!# }
//! ```
//!
//!
//!
//! FallibleI32 - convert `Result<i32, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<i32, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleI32, example);
//!# }
//! ```
//!
//!
//! FallibleU32 - convert `Result<u32, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<u32, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleU32, example);
//!# }
//! ```
//!
//!
//! FallibleDouble - convert `Result<f64, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<f64, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleDouble, example);
//!# }
//! ```
//!
//!
//! FallibleBoolean - convert `Result<bool, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<bool, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleBoolean, example);
//!# }
//! ```
//!
//!
//! FallibleString - convert `Result<String, Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<String, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleString, example);
//!# }
//! ```
//!
//! FallibleU32List - convert `Result<Vec<u32>, Box<dyn MantleResultError>` to [jobject].
//! `Vec<u32>` is converted to java array. Then placed inside `AndroidResult`.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<Vec<u32>, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleU32List, example);
//!# }
//! ```
//!
//!
//! FallibleSerialized(`SerdeType`) - convert `Result<SerdeType, Box<dyn MantleResultError>` to [jobject] (java array of bytes) using serde_json.
//! `SerdeType` is converted to json bytes. Then placed inside `AndroidResult`.
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use serde::{Deserialize, Serialize};
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example() -> Result<ExampleStruct, Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleSerialized(ExampleStruct), example);
//!# }
//! ```
//!
//!
//! FallibleVoid - convert `Result<(), Box<dyn MantleResultError>` to [jobject].
//!
//! ```rust
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::sys::jobject;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<(), Box<dyn MantleResultError>> { unimplemented!() }
//!
//! impl_android_binding!(example_binding(JNIEnv) -> jobject = FallibleVoid, example);
//!# }
//! ```
//!
//!
//! # Example
//!
//!
//! ```rust
//!# use mantle_utilities::error::MantleResultError;
//!# use serde::{Deserialize, Serialize};
//!# use android_binding_macros::impl_android_binding;
//!# use jni::JNIEnv;
//!# use jni::objects::{JObject, JString};
//!# use jni::sys::{jint, jobject};
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example(
//!     serde: ExampleStruct,
//!     number: i32,
//!     bytes: Vec<u8>,
//!     optional: Option<String>,
//! ) -> Result<ExampleStruct, Box<dyn MantleResultError>> {
//!     Ok(ExampleStruct {})
//! }
//!
//! impl_android_binding!(
//!     example_binding(
//!         JNIEnv,
//!         JObject = Serialized(ExampleStruct),
//!         jint,
//!         JObject = Bytes,
//!         JString = OptionalString,
//!     ) -> jobject = FallibleSerialized(ExampleStruct),
//!     example
//! );
//!# }
//! ```

pub use android_binding_macros::*;
pub use android_binding_runtime::*;
