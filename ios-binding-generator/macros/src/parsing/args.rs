use crate::parsing::utils::extract_single_generic_type;
use proc_macro2::Ident;
use strum_macros::{EnumDiscriminants, EnumString};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Token, Type, TypePtr};

#[derive(EnumDiscriminants)]
#[strum_discriminants(vis(pub(self)), derive(EnumString))]
pub enum ArgConversion {
    Primitive,
    String,
    PrimitiveList,
    StringList,
    OptionalPrimitive,
    OptionalString,
    OptionalPrimitiveList,
    Serialized(Type),
    OptionalSerialized(Type),
}

struct MacroArg {
    c_type: Type,
    conversion: Option<ArgConversion>,
}

pub fn parse_binding_args(input: &ParseStream) -> syn::Result<(Vec<Type>, Vec<ArgConversion>)> {
    let content;
    parenthesized!(content in input);
    let args: Punctuated<MacroArg, Token![,]> =
        content.parse_terminated(MacroArg::parse, Token![,])?;
    let args: Vec<MacroArg> = args.into_iter().collect();

    let c_types: Vec<Type> = args.iter().map(|arg| arg.c_type.clone()).collect();
    let conversions = args
        .into_iter()
        .map(|arg| {
            if let Some(conv) = arg.conversion {
                Ok(conv)
            } else {
                infer_conversion(input, &arg.c_type)
            }
        })
        .collect::<syn::Result<Vec<ArgConversion>>>()?;

    Ok((c_types, conversions))
}

impl Parse for ArgConversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let variant: ArgConversionDiscriminants = name
            .to_string()
            .parse()
            .map_err(|_| input.error("unsupported arg conversion"))?;

        Ok(match variant {
            ArgConversionDiscriminants::Primitive => ArgConversion::Primitive,
            ArgConversionDiscriminants::String => ArgConversion::String,
            ArgConversionDiscriminants::PrimitiveList => ArgConversion::PrimitiveList,
            ArgConversionDiscriminants::StringList => ArgConversion::StringList,
            ArgConversionDiscriminants::OptionalPrimitive => ArgConversion::OptionalPrimitive,
            ArgConversionDiscriminants::OptionalString => ArgConversion::OptionalString,
            ArgConversionDiscriminants::OptionalPrimitiveList => {
                ArgConversion::OptionalPrimitiveList
            }
            ArgConversionDiscriminants::Serialized => {
                ArgConversion::Serialized(parse_serialized_type(&input)?)
            }
            ArgConversionDiscriminants::OptionalSerialized => {
                ArgConversion::OptionalSerialized(parse_serialized_type(&input)?)
            }
        })
    }
}

impl Parse for MacroArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let c_type: Type = input.parse()?;
        let conversion: Option<ArgConversion> = if input.peek(Token![=]) {
            let _paren: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(MacroArg { c_type, conversion })
    }
}

fn infer_conversion(input: &ParseStream, c_type: &Type) -> syn::Result<ArgConversion> {
    match c_type {
        Type::Path(_) => Ok(ArgConversion::Primitive),
        Type::Ptr(ptr_ty) => infer_pointer(input, ptr_ty),
        _ => Err(input.error("can't infer specified C type")),
    }
}

fn infer_pointer(input: &ParseStream, ptr_ty: &TypePtr) -> syn::Result<ArgConversion> {
    Ok(if let Type::Path(path) = ptr_ty.elem.as_ref() {
        let path = path.path.segments.last().unwrap();
        if path.ident == "c_char" {
            ArgConversion::String
        } else if path.ident == "MantleList" {
            let inner = extract_single_generic_type(input, &path.arguments)?;
            if let Type::Ptr(_) = inner {
                ArgConversion::StringList
            } else {
                ArgConversion::PrimitiveList
            }
        } else {
            ArgConversion::OptionalPrimitive
        }
    } else {
        return Err(input.error("can't infer specified C pointer conversion"));
    })
}

fn parse_serialized_type(input: &ParseStream) -> syn::Result<Type> {
    let content;
    parenthesized!(content in input);
    content.parse()
}
