use crate::jni_exts::jobject::MantleJObject;
use jni::objects::{GlobalRef, JObject};
use jni::{JNIEnv, JavaVM};
use log::error;

pub struct CallbackStruct {
    pub callback: Option<GlobalRef>,
    pub jvm: Option<JavaVM>,
}

impl CallbackStruct {
    pub const fn new() -> Self {
        Self {
            callback: None,
            jvm: None,
        }
    }

    pub fn with_callback(env: JNIEnv, j_callback: JObject) -> Self {
        let mut callback = CallbackStruct::new();
        callback.update(env, j_callback);
        callback
    }

    pub fn update(&mut self, env: JNIEnv, j_callback: JObject) {
        let callback = Self::global_ref_callback(env, Some(j_callback));
        let jvm = match env.get_java_vm() {
            Ok(jvm) => Some(jvm),
            Err(err) => {
                error!("Error getting jvm from jni env: {:#?}", err);
                None
            }
        };
        self.callback = callback;
        self.jvm = jvm;
    }

    pub fn get_callback_ref(&self) -> Option<(JNIEnv, &GlobalRef)> {
        let Some(jvm) = &self.jvm else {
            return None;
        };
        let Some(callback) = &self.callback else {
            return None;
        };
        let env = jvm
            .attach_current_thread_permanently()
            .unwrap_or_else(|err| {
                error!(
                    "Error getting jvm in spawned thread for polling callback: {:?}",
                    err
                );
                panic!();
            });

        Some((env, callback))
    }

    fn global_ref_callback(env: JNIEnv, callback: Option<JObject>) -> Option<GlobalRef> {
        callback.map(|cb| MantleJObject(cb).to_global_ref(env))
    }
}
