use crate::parsing::args::ArgConversion;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

pub fn gen_rust_args(args: Vec<ArgConversion>) -> TokenStream {
    let args: Vec<TokenStream> = args
        .into_iter()
        .enumerate()
        .map(|(idx, arg)| {
            let ident = format_ident!("_{}", idx);
            match arg {
                ArgConversion::Primitive => gen_primitive_conv(&ident),
                ArgConversion::String => gen_string_ptr_conv(&ident),
                ArgConversion::PrimitiveList => gen_primitive_list_conv(&ident),
                ArgConversion::StringList => gen_string_list_conv(&ident),
                ArgConversion::Serialized(ty) => gen_serde_conv(&ident, &ty),
                ArgConversion::OptionalPrimitive => gen_optional_primitive_conv(&ident),
                ArgConversion::OptionalString => gen_optional_string_conv(&ident),
                ArgConversion::OptionalPrimitiveList => gen_optional_list_conv(&ident),
                ArgConversion::OptionalSerialized(ty) => gen_optional_serde_conv(&ident, &ty),
            }
        })
        .collect();
    quote! {
        #({#args}),*
    }
}

fn gen_primitive_conv(var_name: &Ident) -> TokenStream {
    quote! {
        #var_name
    }
}

fn gen_string_ptr_conv(var_name: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::string_ptr_to_string(#var_name)
    }
}

fn gen_primitive_list_conv(var_name: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::mantle_primitive_list_to_vec(#var_name)
    }
}

fn gen_string_list_conv(var_name: &Ident) -> TokenStream {
    quote! {
       ::ios_binding_generator::mantle_string_list_to_vec(#var_name)
    }
}

fn gen_serde_conv(var_name: &Ident, rust_type: &Type) -> TokenStream {
    quote! {
        match ::ios_binding_generator::from_mantle_bytes_to_serialized::<#rust_type>(#var_name) {
            Ok(r) => r,
            Err(err) => return ::ios_binding_generator::err_to_mantle_result(&err),
        }
    }
}

fn gen_optional_primitive_conv(var_name: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::ptr_to_optional_primitive(#var_name)
    }
}

fn gen_optional_string_conv(var_name: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::string_ptr_to_optional_string(#var_name)
    }
}

fn gen_optional_list_conv(var_name: &Ident) -> TokenStream {
    quote! {
        ::ios_binding_generator::mantle_primitive_list_to_optional_vec(#var_name)
    }
}

fn gen_optional_serde_conv(var_name: &Ident, rust_type: &Type) -> TokenStream {
    quote! {
        match ::ios_binding_generator::from_mantle_bytes_to_optional_serialized::<#rust_type>(#var_name) {
            Ok(r) => r,
            Err(err) => return ::ios_binding_generator::err_to_mantle_result(&err),
        }
    }
}
