//! This crate contains a macro for auto-gen ios bindings.
//!
//! The macro can automatically de/serialize bytes in the C bindings using serde_json.
//!
//! The macro has the following syntax:
//!
//! ```text
//! impl_ios_binding!(
//!     <result binding name>(<C type> = <Argument Conversion>) -> <C return type> = <Output Conversion>,
//!     <a name of the rust function to call with converted args. The result of the function gets converted to the specified C return type>
//! );
//! ```
//! Not de/serialization conversions can be omitted.
//!
//! # Supported Conversions
//!
//! ## Argument Conversion
//!
//!
//! `Primitive` - passes value to rust function as is.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_uint;
//!# fn main() {
//! fn example(number: u32) {}
//!
//! impl_ios_binding!(example_binding(c_uint), example);
//!# }
//! ```
//!
//!
//! `String` - converts `c_char` pointer to [String]. If the pointer is null, an empty string is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_char;
//!# fn main() {
//! fn example(string: String) {}
//!
//! impl_ios_binding!(example_binding(*const c_char), example);
//!# }
//! ```
//!
//!
//! `PrimitiveList` - converts `MantleList<T>` pointer to [Vec]`<T>`. If the pointer is null, an empty [Vec]`<T>` is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# fn main() {
//! fn example(list: Vec<u8>) {}
//!
//! impl_ios_binding!(example_binding(*const MantleList<u8>), example);
//!# }
//! ```
//!
//!
//! `StringList` - converts `MantleList<*const c_char>` pointer to [Vec]`<String>`. If the pointer is null, an empty [Vec]`<String>` is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use std::ffi::c_char;
//!# fn main() {
//! fn example(list: Vec<String>) {}
//!
//! impl_ios_binding!(example_binding(*const MantleList<*const c_char>), example);
//!# }
//! ```
//!
//!
//! `OptionalPrimitive` - Converts `T` pointer to [Option]`<T>`. If the pointer is null, [None] is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_uint;
//!# fn main() {
//! fn example(number: Option<u32>) {}
//!
//! impl_ios_binding!(example_binding(*const c_uint), example);
//!# }
//! ```
//!
//! `OptionalString` - converts `c_char` pointer to [String]. If the pointer is null, [None] is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_char;
//!# fn main() {
//! fn example(string: Option<String>) {}
//!
//! // You have to specify this conversion.
//! impl_ios_binding!(example_binding(*const c_char = OptionalString), example);
//!# }
//! ```
//!
//! `OptionalPrimitiveList` - converts `MantleList<T>` pointer to [Vec]`<T>`. If the pointer is null, [None] is used.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# fn main() {
//! fn example(list: Option<Vec<u8>>) {}
//!
//! impl_ios_binding!(example_binding(*const MantleList<u8> = OptionalPrimitiveList), example);
//!# }
//! ```
//!
//! `Serialized(RustType)` - converts `MantleList<u8>` pointer to `RustType` using serde_json. If the pointer is null, an empty slice `&[]` is passed to serde_json.
//! Can be used in fallible functions only.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use mantle_utilities::error::MantleResultError;
//!# use ios_utilities::MantleResult;
//!# use serde::{Deserialize, Serialize};
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example(list: ExampleStruct) -> Result<(), Box<dyn MantleResultError>> {
//!     Ok(())
//! }
//!
//! impl_ios_binding!(example_binding(*const MantleList<u8> = Serialized(ExampleStruct)) -> MantleResult<()>, example);
//!# }
//! ```
//!
//! `OptionalSerialized(RustType)` - converts `MantleList<u8>` pointer to `RustType` using serde_json. If the pointer is null, [None] is used.
//! Can be used in fallible functions only.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use mantle_utilities::error::MantleResultError;
//!# use ios_utilities::MantleResult;
//!# use serde::{Deserialize, Serialize};
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example(list: Option<ExampleStruct>) -> Result<(), Box<dyn MantleResultError>> {
//!     Ok(())
//! }
//!
//! impl_ios_binding!(example_binding(*const MantleList<u8> = OptionalSerialized(ExampleStruct)) -> MantleResult<()>, example);
//!# }
//! ```
//!
//!
//! ## Output conversion
//!
//!
//! `Primitive` - returns value from rust function as is.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_uint;
//!# fn main() {
//! fn example() -> u32 { 42 }
//!
//! impl_ios_binding!(example_binding() -> c_uint, example);
//!# }
//! ```
//!
//! `String` - converts [String] to `*const c_char`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_char;
//!# fn main() {
//! fn example() -> String { String::new() }
//!
//! impl_ios_binding!(example_binding() -> *const c_char, example);
//!# }
//! ```
//!
//! `PrimitiveList` - converts [Vec]`<T>` to `*const MantleList<T>`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_uchar;
//!# use ios_utilities::MantleList;
//!# fn main() {
//! fn example() -> Vec<u8> { vec![] }
//!
//! impl_ios_binding!(example_binding() -> *const MantleList<u8>, example);
//!# }
//! ```
//!
//!
//! `Void` - return nothing.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# fn main() {
//! fn example() {}
//!
//! impl_ios_binding!(example_binding(), example);
//!# }
//! ```
//!
//! `FalliblePrimitive` - converts `Result<T, MantleResultError>` to `MantleResult<T>`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use std::ffi::c_uchar;
//!# use ios_utilities::MantleList;
//!# use std::ffi::c_uint;
//!# use ios_utilities::MantleResult;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<u32, Box<dyn MantleResultError>> { Ok(42) }
//!
//! impl_ios_binding!(example_binding() -> MantleResult<c_uint>, example);
//!# }
//! ```
//!
//! `FallibleString` - converts `Result<String, MantleResultError>` to `MantleResult<*const c_char>`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use ios_utilities::MantleResult;
//!# use std::ffi::c_char;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<String, Box<dyn MantleResultError>> { Ok(String::new()) }
//!
//! impl_ios_binding!(example_binding() -> MantleResult<*const c_char>, example);
//!# }
//! ```
//!
//! `FalliblePrimitiveList` - converts `Result<Vec<T>, MantleResultError>` to `MantleResult<MantleList<T>>`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use ios_utilities::MantleResult;
//!# use std::ffi::c_uchar;
//!# use mantle_utilities::error::MantleResultError;
//!# fn main() {
//! fn example() -> Result<Vec<u8>, Box<dyn MantleResultError>> { Ok(vec![]) }
//!
//! impl_ios_binding!(example_binding() -> MantleResult<MantleList<c_uchar>>, example);
//!# }
//! ```
//!
//! `FallibleSerialized(RustType)` - converts `Result<RustType, MantleResultError>` to `MantleResult<MantleList<u8>>` using serde_json.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use mantle_utilities::error::MantleResultError;
//!# use ios_utilities::MantleResult;
//!# use serde::{Deserialize, Serialize};
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example() -> Result<ExampleStruct, Box<dyn MantleResultError>> {
//!     Ok(ExampleStruct {})
//! }
//!
//! impl_ios_binding!(example_binding() -> MantleResult<MantleList<u8>> = FallibleSerialized(ExampleStruct), example);
//!# }
//! ```
//!
//! `FallibleVoid(RustType)` - converts `Result<(), MantleResultError>` to `MantleResult<()>`.
//!
//! ```rust
//!# use ios_binding_generator::impl_ios_binding;
//!# use ios_utilities::MantleList;
//!# use mantle_utilities::error::MantleResultError;
//!# use ios_utilities::MantleResult;
//!# use serde::{Deserialize, Serialize};
//!# fn main() {
//! #[derive(Deserialize, Serialize)]
//! struct ExampleStruct {}
//!
//! fn example() -> Result<(), Box<dyn MantleResultError>> {
//!     Ok(())
//! }
//!
//! impl_ios_binding!(example_binding() -> MantleResult<()>, example);
//!# }
//! ```
//!
//!
/// # Example
///
///
/// ```rust
///# use ios_binding_generator::impl_ios_binding;
///# use ios_utilities::MantleList;
///# use mantle_utilities::error::MantleResultError;
///# use ios_utilities::MantleResult;
///# use std::ffi::{c_uchar, c_char};
///# use serde::{Deserialize, Serialize};
///# fn main() {
/// #[derive(Deserialize, Serialize)]
/// struct ExampleStruct {}
///
/// fn example(
///     serde: ExampleStruct,
///     primitive: u8,
///     list: Vec<u8>,
///     optional: Option<String>,
/// ) -> Result<ExampleStruct, Box<dyn MantleResultError>> {
///     Ok(ExampleStruct {})
/// }
///
/// impl_ios_binding!(
///     example_binding(
///         *const MantleList<u8> = Serialized(ExampleStruct),
///         c_uchar, *const MantleList<u8>,
///         *const c_char = OptionalString
///     ) -> MantleResult<MantleList<u8>> = FallibleSerialized(ExampleStruct),
///     example
/// );
///# }
/// ```
pub use ios_binding_macros::*;
pub use ios_binding_runtime::*;
