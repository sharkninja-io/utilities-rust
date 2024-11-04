use jni::{sys::jbyteArray, JNIEnv};
use log::error;

pub struct AndroidData;

impl AndroidData {
    pub fn to_jbyte_array(slice: &[u8], jni_env: JNIEnv) -> jbyteArray {
        jni_env.byte_array_from_slice(slice).unwrap_or_else(|err| {
            error!("Error creating jbytearray: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }

    pub fn jbyte_array_to_vec(byte_array: jbyteArray, jni_env: JNIEnv) -> Vec<u8> {
        jni_env
            .convert_byte_array(byte_array)
            .unwrap_or_else(|err| {
                error!("Error converting jbyteArray to Vec<u8>: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            })
    }
}
