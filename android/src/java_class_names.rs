use crate::jni_exts::jobject::MantleJObject;
use crate::jni_exts::string::AndroidString;
use crate::traits::JavaClass;
use jni::objects::{GlobalRef, JClass, JObject};
use jni::JNIEnv;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// This is initialized lazily because ctor does not guarantee which ctor function will be called first.
pub static CLASSNAMES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![
        "java/lang/Integer".to_string(), // Integer
        "java/lang/Double".to_string(),  // Double
        "java/lang/Boolean".to_string(), // Boolean
        "java/lang/Long".to_string(),    // Long
        "kotlin/UInt".to_string(),       // UInt
    ])
});

pub static CLASSREFSMAP: Lazy<Mutex<HashMap<String, GlobalRef>>> = Lazy::new(Default::default);

pub fn get_class<Rust, T>(instance: Option<&T>) -> JClass
where
    T: JavaClass<Rust>,
{
    let full_name = T::full_name(instance);
    get_class_from_name(full_name)
}

pub fn get_class_from_name(full_name: String) -> JClass<'static> {
    let map = CLASSREFSMAP.lock().unwrap();
    let class = map
        .get(full_name.as_str())
        .unwrap_or_else(|| {
            error!("{} not found in Class Reference Map", full_name);
            panic!();
        })
        .to_owned();
    drop(map);
    JClass::from(class.as_obj().into_inner())
}

pub fn capture_class_refs(env: JNIEnv) {
    // Classes are stored on the stack. In a separate thread the app's stack is gone. So the JVM will
    // just look in the default 'system' class loader:
    // https://developer.android.com/training/articles/perf-jni#faq_FindClass

    // If using a class defined in Java/Kotlin, add it's full class name to this list in a ctor function
    let names = CLASSNAMES.lock().unwrap();

    let mut map = CLASSREFSMAP.lock().unwrap();
    for name in names.iter() {
        if !map.contains_key(name) {
            let class = JObject::from(AndroidString(name.to_string()).to_jclass(env));
            let class_ref: GlobalRef = MantleJObject(class).to_global_ref(env);
            map.insert(name.to_string(), class_ref);
        }
    }
}
