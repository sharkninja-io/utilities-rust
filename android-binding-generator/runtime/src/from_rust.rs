use android_utilities::jni_exts::bool::bool_to_bool_class;
use android_utilities::jni_exts::byte_array::AndroidData;
use android_utilities::jni_exts::double::f64_to_double_class;
use android_utilities::jni_exts::int::i32_to_integer_class;
use android_utilities::jni_exts::long::i64_to_long_class;
use android_utilities::{error_result, success_result, AndroidResult};
use jni::objects::JValue;
use jni::sys::{jboolean, jint, jlong, jobject, jsize};
use jni::JNIEnv;
use log::error;
use mantle_utilities::error::MantleResultError;
use serde::Serialize;

type MantleResult<T> = Result<T, Box<dyn MantleResultError>>;

pub fn i64_to_jlong(value: i64) -> jlong {
    value
}

pub fn i32_to_jint(value: i32) -> jint {
    value
}

pub fn bool_to_jboolean(value: bool) -> jboolean {
    jboolean::from(value)
}

pub fn string_to_jobject(env: JNIEnv, value: String) -> jobject {
    env.new_string(value)
        .expect("couldn't create java string")
        .into_inner()
}

pub fn bytes_to_jobject(env: JNIEnv, bytes: Vec<u8>) -> jobject {
    AndroidData::to_jbyte_array(&bytes, env)
}

pub fn fallible_i64_to_jobject(env: JNIEnv, result: MantleResult<i64>) -> jobject {
    match result {
        Ok(v) => *success_result(env, i64_to_long_class(env, v)),
        Err(err) => *error_result(env, err),
    }
}

pub fn fallible_i32_to_jobject(env: JNIEnv, result: MantleResult<i32>) -> jobject {
    match result {
        Ok(v) => *success_result(env, i32_to_integer_class(env, v)),
        Err(err) => *error_result(env, err),
    }
}

pub fn fallible_u32_to_jobject(env: JNIEnv, result: MantleResult<u32>) -> jobject {
    fallible_i32_to_jobject(env, result.map(|v| v as i32))
}

pub fn fallible_double_to_jobject(env: JNIEnv, result: MantleResult<f64>) -> jobject {
    match result {
        Ok(v) => *success_result(env, f64_to_double_class(env, v)),
        Err(err) => *error_result(env, err),
    }
}

pub fn fallible_bool_to_jobject(env: JNIEnv, result: MantleResult<bool>) -> jobject {
    match result {
        Ok(v) => *success_result(env, bool_to_bool_class(env, v)),
        Err(err) => *error_result(env, err),
    }
}

pub fn fallible_string_to_jobject(env: JNIEnv, result: MantleResult<String>) -> jobject {
    match result {
        Ok(v) => {
            let jni_string = env.new_string(v).expect("couldn't create java string");
            *success_result(env, JValue::from(jni_string.into_inner()))
        }
        Err(err) => *error_result(env, err),
    }
}

pub fn fallible_vec_u32_to_jobject(env: JNIEnv, result: MantleResult<Vec<u32>>) -> jobject {
    let vec = match result {
        Ok(v) => v,
        Err(err) => return mantle_boxed_err_to_jobject(env, err),
    };

    let array = env.new_int_array(vec.len() as jsize).unwrap_or_else(|err| {
        error!("Could not create int array: {:?}", err);
        env.exception_describe().unwrap();
        panic!();
    });
    let jint_array = vec
        .into_iter()
        .map(|int| int as jint)
        .collect::<Vec<jint>>();
    env.set_int_array_region(array, 0, jint_array.as_slice())
        .unwrap_or_else(|err| {
            error!("Could not copy vec contents into int array: {:?}", err);
            env.exception_describe().unwrap();
            panic!();
        });
    *success_result(env, JValue::from(array))
}

pub fn fallible_void_to_jobject(env: JNIEnv, result: MantleResult<()>) -> jobject {
    *AndroidResult(result).to_jobject_result(env)
}

pub fn fallible_serialized_to_jobject<T: Serialize>(
    env: JNIEnv,
    result: MantleResult<T>,
) -> jobject {
    let success_value = match result {
        Ok(v) => v,
        Err(err) => return mantle_boxed_err_to_jobject(env, err),
    };

    let serialized = match serde_json::to_vec(&success_value) {
        Ok(bytes) => bytes,
        Err(err) => return mantle_err_to_jobject(env, err),
    };

    *success_result(
        env,
        JValue::from(AndroidData::to_jbyte_array(&serialized, env)),
    )
}

pub fn mantle_boxed_err_to_jobject(env: JNIEnv, err: Box<dyn MantleResultError>) -> jobject {
    *error_result(env, err)
}

pub fn mantle_err_to_jobject<E: MantleResultError + 'static>(env: JNIEnv, err: E) -> jobject {
    *error_result(env, Box::new(err))
}
