//! Library for compile-time encryption of config variables
//! for their later decryption at runtime.
//!
//! Uses AES-GCM to encrypt data.
//!
//! # Usage
//! ```
//! use confenc::confenc;
//! assert_eq!("value", confenc!("confenc/tests/config.yml", "key"));
//! assert_eq!("nested value", confenc!("confenc/tests/config.yml", "nested.key"));
//! ```

pub use confenc_crydec::*;
pub use confenc_macros::*;
