use crate::parsing::args::ArgConversion;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

pub fn gen_rust_args(args: Vec<ArgConversion>, env_var_name: &Ident) -> TokenStream {
    let args: Vec<TokenStream> = args
        .into_iter()
        .enumerate()
        .map(|(idx, arg)| {
            let ident = format_ident!("_{}", idx);
            match arg {
                ArgConversion::I64 => gen_i64_conversion(&ident),
                ArgConversion::I32 => gen_i32_conversion(&ident),
                ArgConversion::U32 => gen_u32_conversion(&ident),
                ArgConversion::U8 => gen_u8_conversion(&ident),
                ArgConversion::Double => gen_double_conversion(&ident),
                ArgConversion::Boolean => gen_boolean_conversion(&ident),
                ArgConversion::String => gen_string_conversion(&ident, env_var_name),
                ArgConversion::Bytes => gen_bytes_conversion(&ident, env_var_name),
                ArgConversion::StringList => gen_string_list_conversion(&ident, env_var_name),
                ArgConversion::Serialized(ty) => {
                    gen_serialized_conversion(&ident, env_var_name, &ty)
                }
                ArgConversion::OptionalU32 => gen_optional_u32(&ident, env_var_name),
                ArgConversion::OptionalI32 => gen_optional_i32(&ident, env_var_name),
                ArgConversion::OptionalBool => gen_optional_bool(&ident, env_var_name),
                ArgConversion::OptionalString => gen_optional_string(&ident, env_var_name),
                ArgConversion::OptionalSerialized(ty) => {
                    gen_optional_serialized_conversion(&ident, env_var_name, &ty)
                }
            }
        })
        .collect();
    quote! {
        #({#args}),*
    }
}

fn gen_i64_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jlong_to_i64(#var_name)
    }
}

fn gen_i32_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jint_to_i32(#var_name)
    }
}

fn gen_u32_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jint_to_u32(#var_name)
    }
}

fn gen_u8_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jint_to_u8(#var_name)
    }
}

fn gen_double_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jdouble_to_f64(#var_name)
    }
}

fn gen_boolean_conversion(var_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jboolean_to_bool(#var_name)
    }
}

fn gen_string_conversion(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jstring_to_string(#env_name, #var_name)
    }
}

fn gen_bytes_conversion(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jobject_to_bytes(#env_name, #var_name)
    }
}

fn gen_string_list_conversion(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jobject_to_string_vec(#env_name, #var_name)
    }
}

fn gen_serialized_conversion(var_name: &Ident, env_name: &Ident, rust_type: &Type) -> TokenStream {
    quote! {
        match ::android_binding_generator::jobject_to_serialized::<#rust_type>(#env_name, #var_name) {
            Ok(v) => v,
            Err(err) => return ::android_binding_generator::mantle_err_to_jobject(#env_name, err),
        }
    }
}

fn gen_optional_u32(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jobject_to_optional_u32(#env_name, #var_name)
    }
}

fn gen_optional_i32(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jobject_to_optional_i32(#env_name, #var_name)
    }
}

fn gen_optional_bool(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jobject_to_optional_bool(#env_name, #var_name)
    }
}

fn gen_optional_string(var_name: &Ident, env_name: &Ident) -> TokenStream {
    quote! {
        ::android_binding_generator::jstring_to_optional_string(#env_name, #var_name)
    }
}

fn gen_optional_serialized_conversion(
    var_name: &Ident,
    env_name: &Ident,
    rust_type: &Type,
) -> TokenStream {
    quote! {
        match ::android_binding_generator::jobject_to_optional_serialized::<#rust_type>(#env_name, #var_name) {
            Ok(v) => v,
            Err(err) => return ::android_binding_generator::mantle_err_to_jobject(#env_name, err),
        }
    }
}
