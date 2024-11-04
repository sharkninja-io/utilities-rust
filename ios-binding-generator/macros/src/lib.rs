use crate::generating::args::gen_rust_args;
use crate::generating::output::gen_return_expr;
use crate::parsing::MacroInput;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod generating;
mod parsing;

/// # Auto-gen ios bindings.
///
///
/// ## Macro Syntax
///
/// ```text
/// impl_ios_binding!(
///     <C binding name>(<C type> = <Argument Conversion>) -> <C return type> = <Output Conversion>,
///     <rust function>
/// );
/// ```
///
/// ### Argument conversion
///
/// - Primitive
/// - String
/// - PrimitiveList
/// - StringList
/// - OptionalPrimitive
/// - OptionalString
/// - OptionalPrimitiveList
/// - Serialized(`RustType`)
/// - OptionalSerialized(`RustType`)
///
/// ### Output conversion
///
/// - Primitive
/// - String
/// - PrimitiveList,
/// - Void,
/// - FalliblePrimitive,
/// - FallibleString,
/// - FalliblePrimitiveList,
/// - FallibleSerialized(Type),
/// - FallibleVoid,
#[proc_macro]
pub fn impl_ios_binding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let MacroInput {
        binding_name,
        binding_args,
        binding_return_type,
        rust_function_name,
        arg_conversions,
        output_conversion,
    } = input;
    let rust_args = gen_rust_args(arg_conversions);
    let result_ident = format_ident!("_result");
    let binding_args = binding_args.into_iter().enumerate().map(|(idx, ty)| {
        let arg_name = format_ident!("_{}", idx);
        quote! {
            #arg_name : #ty
        }
    });
    let return_expr = gen_return_expr(output_conversion, &result_ident);
    (quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #binding_name(#(#binding_args),*) #binding_return_type  {
            let #result_ident = #rust_function_name(#rust_args);
            #return_expr
        }
    })
    .into()
}
