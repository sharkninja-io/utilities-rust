use crate::parsing::utils::{
    extract_last_segment, extract_last_segment_from_type, extract_single_generic_type,
};
use proc_macro2::Ident;
use strum_macros::{EnumDiscriminants, EnumString};
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, ReturnType, Token, Type, TypePath};

#[derive(EnumDiscriminants)]
#[strum_discriminants(vis(pub(self)), derive(EnumString))]
pub enum OutputConversion {
    Primitive,
    String,
    PrimitiveList,
    Void,
    FalliblePrimitive,
    FallibleString,
    FalliblePrimitiveList,
    FallibleSerialized(Type),
    FallibleVoid,
}

pub fn parse_binding_output(input: &ParseStream) -> syn::Result<(ReturnType, OutputConversion)> {
    let return_type: ReturnType = input.parse()?;
    let c_return_type = match &return_type {
        ReturnType::Default => return Ok((return_type, OutputConversion::Void)),
        ReturnType::Type(_, ty) => ty.as_ref(),
    };

    let conversion: OutputConversion = if input.peek(Token![=]) {
        let _: Token![=] = input.parse()?;
        input.parse()?
    } else {
        infer_output(input, c_return_type)?
    };

    Ok((return_type, conversion))
}

impl Parse for OutputConversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let variant: OutputConversionDiscriminants = name
            .to_string()
            .parse()
            .map_err(|_| input.error("unsupported output conversion"))?;
        Ok(match variant {
            OutputConversionDiscriminants::Primitive => OutputConversion::Primitive,
            OutputConversionDiscriminants::String => OutputConversion::String,
            OutputConversionDiscriminants::PrimitiveList => OutputConversion::PrimitiveList,
            OutputConversionDiscriminants::Void => OutputConversion::Void,
            OutputConversionDiscriminants::FalliblePrimitive => OutputConversion::FalliblePrimitive,
            OutputConversionDiscriminants::FallibleString => OutputConversion::FallibleString,
            OutputConversionDiscriminants::FalliblePrimitiveList => {
                OutputConversion::FalliblePrimitiveList
            }
            OutputConversionDiscriminants::FallibleVoid => OutputConversion::FallibleVoid,
            OutputConversionDiscriminants::FallibleSerialized => {
                let content;
                parenthesized!(content in input);
                let ty: Type = content.parse()?;
                OutputConversion::FallibleSerialized(ty)
            }
        })
    }
}

fn infer_output(input: &ParseStream, c_type: &Type) -> syn::Result<OutputConversion> {
    Ok(match c_type {
        Type::Path(path) => {
            let last_segment = extract_last_segment(&path.path);
            if last_segment.ident == "MantleResult" {
                let generic = extract_single_generic_type(input, &last_segment.arguments)?;
                infer_fallible_output(input, generic)?
            } else {
                OutputConversion::Primitive
            }
        }
        Type::Ptr(ptr) => {
            let last_segment = extract_last_segment_from_type(input, ptr.elem.as_ref())?;
            if last_segment.ident == "c_char" {
                OutputConversion::String
            } else if last_segment.ident == "MantleList" {
                OutputConversion::PrimitiveList
            } else {
                return Err(input.error("can't infer C return pointer type"));
            }
        }
        _ => return Err(input.error("can't infer C return type")),
    })
}

fn infer_fallible_output(
    input: &ParseStream,
    c_inner_type: &Type,
) -> syn::Result<OutputConversion> {
    Ok(match c_inner_type {
        Type::Path(path) => infer_fallible_type_path(input, path)?,
        Type::Ptr(ptr) => {
            let last_segment = extract_last_segment_from_type(input, ptr.elem.as_ref())?;
            if last_segment.ident == "c_char" {
                OutputConversion::FallibleString
            } else {
                return Err(input.error("can't infer fallible C return pointer type"));
            }
        }
        Type::Tuple(_) => OutputConversion::FallibleVoid,
        _ => return Err(input.error("can't infer fallible C return type")),
    })
}

fn infer_fallible_type_path(_: &ParseStream, path: &TypePath) -> syn::Result<OutputConversion> {
    let last_segment = extract_last_segment(&path.path);
    Ok(if last_segment.ident == "MantleList" {
        OutputConversion::FalliblePrimitiveList
    } else {
        OutputConversion::FalliblePrimitive
    })
}
