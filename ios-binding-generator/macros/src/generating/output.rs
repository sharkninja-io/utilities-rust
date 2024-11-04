use crate::parsing::output::OutputConversion;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;

pub fn gen_return_expr(return_type: OutputConversion, result_ident: &Ident) -> TokenStream {
    match return_type {
        OutputConversion::Primitive => gen_primitive_return_expr(result_ident),
        OutputConversion::String => gen_string_return_expr(result_ident),
        OutputConversion::PrimitiveList => gen_primitive_list_return_expr(result_ident),
        OutputConversion::Void => gen_void_return_expr(result_ident),
        OutputConversion::FalliblePrimitive => gen_fallible_primitive_return_expr(result_ident),
        OutputConversion::FallibleString => gen_fallible_string_return_expr(result_ident),
        OutputConversion::FalliblePrimitiveList => {
            gen_fallible_primitive_list_return_expr(result_ident)
        }
        OutputConversion::FallibleSerialized(ty) => gen_serialized_return_expr(result_ident, &ty),
        OutputConversion::FallibleVoid => gen_fallible_void_return_expr(result_ident),
    }
}

fn gen_primitive_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        #result_ident
    }
}

fn gen_string_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::string_to_string_ptr(#result_ident)
    }
}

fn gen_primitive_list_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::vec_to_primitive_list(#result_ident)
    }
}

fn gen_void_return_expr(_result_ident: &Ident) -> TokenStream {
    quote! {}
}

fn gen_fallible_primitive_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::primitive_result_to_mantle_result(#result_ident)
    }
}

fn gen_fallible_string_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::string_result_to_mantle_result(#result_ident)
    }
}

fn gen_fallible_primitive_list_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::list_result_to_mantle_result(#result_ident)
    }
}

fn gen_serialized_return_expr(result_ident: &Ident, _rust_type: &Type) -> TokenStream {
    quote! {
        ::ios_binding_generator::serialized_result_to_mantle_result(#result_ident)
    }
}

fn gen_fallible_void_return_expr(result_ident: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::void_result_to_mantle_result(#result_ident)
    }
}
