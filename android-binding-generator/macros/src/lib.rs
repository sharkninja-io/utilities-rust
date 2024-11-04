use crate::generating::args::gen_rust_args;
use crate::generating::output::gen_return_expr;
use crate::parsing::MacroInput;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod generating;
mod parsing;

/// # Auto-gen android bindings.
///
///
/// ## Macro Syntax
///
/// ```text
/// impl_android_binding!(
///         JNIEnv, JClass(Optional), <JNI binding name>(<JNI type> = <Argument Conversion>
///     ) -> <JNI return type> = <Output Conversion>,
///     <rust function>
/// );
/// ```
///
/// ### Argument conversion
///
/// - I64,
/// - I32,
/// - U32,
/// - U8,
/// - Double,
/// - Boolean,
/// - String,
/// - Bytes,
/// - StringList,
/// - Serialized(`SerdeType`),
/// - OptionalI32,
/// - OptionalU32,
/// - OptionalBool,
/// - OptionalString,
/// - OptionalSerialized(`SerdeType`),
///
/// ### Output conversion
///
/// - I64,
/// - I32,
/// - Boolean,
/// - String,
/// - Bytes,
/// - Void,
/// - FallibleI64,
/// - FallibleI32,
/// - FallibleU32,
/// - FallibleDouble,
/// - FallibleBoolean,
/// - FallibleString,
/// - FallibleU32List,
/// - FallibleSerialized(`SerdeType`),
/// - FallibleVoid,
#[proc_macro]
pub fn impl_android_binding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let MacroInput {
        binding_name,
        binding_return_type,
        rust_function_name,
        binding_arg_info,
        output_conversion,
    } = input;
    let jni_env_name = format_ident!("env");
    let rust_args = gen_rust_args(binding_arg_info.conversions, &jni_env_name);
    let result_ident = format_ident!("_result");
    let jni_env = binding_arg_info.jni_env;
    let jni_env = quote! {
        #jni_env_name: #jni_env
    };
    let jclass = binding_arg_info.jclass;
    let jclass = if let Some(jclass) = jclass {
        quote! {
            _class: #jclass,
        }
    } else {
        quote! {}
    };

    let binding_args = binding_arg_info
        .rest
        .into_iter()
        .enumerate()
        .map(|(idx, ty)| {
            let arg_name = format_ident!("_{}", idx);
            quote! {
                #arg_name : #ty
            }
        });
    let return_expr = gen_return_expr(output_conversion, &result_ident, &jni_env_name);
    (quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #binding_name(#jni_env, #jclass #(#binding_args),*) #binding_return_type  {
            let #result_ident = #rust_function_name(#rust_args);
            #return_expr
        }
    })
    .into()
}
