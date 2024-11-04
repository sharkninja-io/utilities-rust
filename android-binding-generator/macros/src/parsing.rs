use crate::parsing::args::BindingArgs;
use crate::parsing::output::OutputConversion;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Path, ReturnType, Token};

pub mod args;
pub mod output;

mod utils;

pub struct MacroInput {
    pub binding_name: Ident,
    pub binding_return_type: ReturnType,
    pub rust_function_name: Path,
    pub binding_arg_info: BindingArgs,
    pub output_conversion: OutputConversion,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let binding_name: Ident = input.parse()?;
        let binding_arg_info = args::parse_binding_args(&input)?;
        let (binding_return_type, output_conversion) = output::parse_binding_output(&input)?;
        let rust_function_name = parse_rust_function_name(&input)?;

        Ok(MacroInput {
            binding_name,
            binding_return_type,
            rust_function_name,
            binding_arg_info,
            output_conversion,
        })
    }
}

fn parse_rust_function_name(input: &ParseStream) -> syn::Result<Path> {
    let _comma: Token![,] = input.parse()?;
    input.parse()
}
