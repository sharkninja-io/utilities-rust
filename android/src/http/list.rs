use crate::java_class_names;
use crate::traits::JavaClass;
use jni::objects::{JClass, JObject};
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;

pub struct AndroidList<T>(pub Vec<T>);

impl<T> AndroidList<T> {
    pub fn into_jobject<Rust>(self, jni_env: JNIEnv) -> jobject
    where
        T: JavaClass<Rust>,
    {
        let list = &self.0;
        let instance: Option<&T> = if list.is_empty() {
            // If an empty list is passed then there is no instance to reference
            None
        } else {
            let first = list.get(0).unwrap();
            Some(first)
        };
        let j_class = java_class_names::get_class(instance);
        AndroidList::jobject_using_class(list, jni_env, j_class)
    }

    pub fn jobject_using_class<Rust>(list: &Vec<T>, jni_env: JNIEnv, j_class: JClass) -> jobject
    where
        T: JavaClass<Rust>,
    {
        if list.is_empty() {
            return AndroidList::empty_jobject(list, jni_env, j_class);
        }
        let first = list.get(0).unwrap();
        let first_object = first.j_object(jni_env, j_class);
        let array = jni_env
            .new_object_array(list.len() as i32, j_class, first_object)
            .unwrap_or_else(|err| {
                error!(
                    "Could not create an array for class: {}: {:?}",
                    T::full_name(Some(first)),
                    err
                );
                jni_env.exception_describe().unwrap();
                panic!();
            });
        for (index, object) in list.iter().skip(1).enumerate() {
            let j_object = object.j_object(jni_env, java_class_names::get_class(Some(object)));
            jni_env
                .set_object_array_element(array, index as i32, j_object)
                .unwrap_or_else(|err| {
                    error!(
                        "Could not add {} element to array: {:?}",
                        T::full_name(Some(object)),
                        err
                    );
                    jni_env.exception_describe().unwrap();
                    panic!();
                });
        }
        array
    }

    pub fn empty_jobject<Rust>(_list: &[T], jni_env: JNIEnv, j_class: JClass) -> jobject
    where
        T: JavaClass<Rust>,
    {
        jni_env
            .new_object_array(0, j_class, JObject::null())
            .unwrap_or_else(|err| {
                error!(
                    "Could not create an array for class: {}: {:?}",
                    T::full_name(None),
                    err
                );
                jni_env.exception_describe().unwrap();
                panic!();
            })
    }
}
