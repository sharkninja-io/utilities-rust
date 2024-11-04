use jni::objects::{GlobalRef, JObject};
use jni::JNIEnv;
use log::error;

pub struct MantleJObject<'a>(pub JObject<'a>);

impl<'a> MantleJObject<'a> {
    pub fn to_global_ref(&self, jni_env: JNIEnv) -> GlobalRef {
        jni_env.new_global_ref(self.0).unwrap_or_else(|err| {
            error!("Error getting j_object from global ref: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }
}

#[cfg(feature = "http")]
impl<'a> MantleJObject<'a> {
    pub(crate) fn to_unsigned_int_field(&self, jni_env: JNIEnv, name: &str) -> u32 {
        jni_env
            .get_field(self.0, name, crate::java_signatures::INT_SIG)
            .unwrap_or_else(|err| {
                error!("Error getting unsigned int field: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            })
            .i()
            .unwrap_or_else(|err| {
                error!("Error converting unsigned int field to jint: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            }) as u32
    }

    pub(crate) fn to_byte_array_field(&self, jni_env: JNIEnv, name: &str) -> Vec<u8> {
        let object = jni_env
            .get_field(
                self.0,
                name,
                ["[", crate::java_signatures::BYTE_SIG].concat(),
            )
            .unwrap_or_else(|err| {
                error!("Error getting byte array field: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            })
            .l()
            .unwrap_or_else(|err| {
                error!("Error converting byte array to jobject: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        jni_env.convert_byte_array(*object).unwrap_or_else(|err| {
            error!("Error converting jobject to byte array: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }
}
