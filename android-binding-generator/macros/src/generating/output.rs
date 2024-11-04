use crate::parsing::output::OutputConversion;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn gen_return_expr(
    return_type: OutputConversion,
    result_ident: &Ident,
    jni_env_var_name: &Ident,
) -> TokenStream {
    match return_type {
        OutputConversion::I64 => gen_i64_return_expr(result_ident),
        OutputConversion::I32 => gen_i32_return_expr(result_ident),
        OutputConversion::Boolean => gen_boolean_return_expr(result_ident),
        OutputConversion::String => gen_string_return_expr(result_ident, jni_env_var_name),
        OutputConversion::Bytes => gen_bytes_return_expr(result_ident, jni_env_var_name),
        OutputConversion::Void => gen_void_return_expr(),
        OutputConversion::FallibleI64 => {
            gen_fallible_i64_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleI32 => {
            gen_fallible_i32_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleDouble => {
            gen_fallible_double_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleBoolean => {
            gen_fallible_boolean_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleString => {
            gen_fallible_string_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleU32 => {
            gen_fallible_u32_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleU32List => {
            gen_fallible_u32_list_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleSerialized(_) => {
            gen_fallible_serializable_return_expr(result_ident, jni_env_var_name)
        }
        OutputConversion::FallibleVoid => {
            gen_fallible_void_return_expr(result_ident, jni_env_var_name)
        }
    }
}

pub fn gen_i64_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::i64_to_jlong(#result_ident)
    }
}

pub fn gen_i32_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::i32_to_jint(#result_ident)
    }
}

pub fn gen_boolean_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::bool_to_jboolean(#result_ident)
    }
}

pub fn gen_string_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::string_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_bytes_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::bytes_to_jobject(#env_name, #result_ident)
    }
}

fn gen_void_return_expr() -> TokenStream {
    quote! {}
}

pub fn gen_fallible_i64_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_i64_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_i32_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_i32_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_u32_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_u32_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_double_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_double_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_boolean_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_bool_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_string_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_string_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_u32_list_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_vec_u32_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_serializable_return_expr(
    result_ident: &Ident,
    env_name: &Ident,
) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_serialized_to_jobject(#env_name, #result_ident)
    }
}

pub fn gen_fallible_void_return_expr(result_ident: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::fallible_void_to_jobject(#env_name, #result_ident)
    }
}
