use crate::parsing::args::ArgConversion;
use crate::parsing::output::OutputConversion;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Path, ReturnType, Token, Type};

pub mod args;
pub mod output;

mod utils;

pub struct MacroInput {
    pub binding_name: Ident,
    pub binding_args: Vec<Type>,
    pub binding_return_type: ReturnType,
    pub rust_function_name: Path,
    pub arg_conversions: Vec<ArgConversion>,
    pub output_conversion: OutputConversion,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let binding_name: Ident = input.parse()?;
        let (binding_args, arg_conversions) = args::parse_binding_args(&input)?;
        let (binding_return_type, output_conversion) = output::parse_binding_output(&input)?;
        let rust_function_name = parse_rust_function_name(&input)?;

        Ok(MacroInput {
            binding_name,
            binding_args,
            binding_return_type,
            rust_function_name,
            arg_conversions,
            output_conversion,
        })
    }
}

fn parse_rust_function_name(input: &ParseStream) -> syn::Result<Path> {
    let _comma: Token![,] = input.parse()?;
    input.parse()
}
