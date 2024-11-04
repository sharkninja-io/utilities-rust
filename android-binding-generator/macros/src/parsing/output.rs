use crate::parsing::output::OutputConversion::FallibleVoid;
use crate::parsing::utils::extract_last_segment;
use proc_macro2::Ident;
use strum_macros::{EnumDiscriminants, EnumString};
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, ReturnType, Token, Type};

#[derive(EnumDiscriminants)]
#[strum_discriminants(vis(pub(self)), derive(EnumString))]
pub enum OutputConversion {
    I64,
    I32,
    Boolean,
    String,
    Bytes,
    Void,
    FallibleI64,
    FallibleI32,
    FallibleU32,
    FallibleDouble,
    FallibleBoolean,
    FallibleString,
    FallibleU32List,
    FallibleSerialized(Type),
    FallibleVoid,
}

pub fn parse_binding_output(input: &ParseStream) -> syn::Result<(ReturnType, OutputConversion)> {
    let return_type: ReturnType = input.parse()?;
    let jni_return_type = match &return_type {
        ReturnType::Default => return Ok((return_type, OutputConversion::Void)),
        ReturnType::Type(_, ty) => ty.as_ref(),
    };

    let conversion: OutputConversion = if input.peek(Token![=]) {
        let _: Token![=] = input.parse()?;
        input.parse()?
    } else {
        infer_output(input, jni_return_type)?
    };

    Ok((return_type, conversion))
}

impl Parse for OutputConversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let variant: OutputConversionDiscriminants = name
            .to_string()
            .parse()
            .map_err(|_| input.error("unknown output conversion"))?;

        Ok(match variant {
            OutputConversionDiscriminants::I64 => OutputConversion::I64,
            OutputConversionDiscriminants::I32 => OutputConversion::I32,
            OutputConversionDiscriminants::FallibleDouble => OutputConversion::FallibleDouble,
            OutputConversionDiscriminants::Boolean => OutputConversion::Boolean,
            OutputConversionDiscriminants::String => OutputConversion::String,
            OutputConversionDiscriminants::Bytes => OutputConversion::Bytes,
            OutputConversionDiscriminants::Void => OutputConversion::Void,
            OutputConversionDiscriminants::FallibleI64 => OutputConversion::FallibleI64,
            OutputConversionDiscriminants::FallibleI32 => OutputConversion::FallibleI32,
            OutputConversionDiscriminants::FallibleU32 => OutputConversion::FallibleU32,
            OutputConversionDiscriminants::FallibleBoolean => OutputConversion::FallibleBoolean,
            OutputConversionDiscriminants::FallibleString => OutputConversion::FallibleString,
            OutputConversionDiscriminants::FallibleU32List => OutputConversion::FallibleU32List,
            OutputConversionDiscriminants::FallibleSerialized => {
                let content;
                parenthesized!(content in input);
                let ty: Type = content.parse()?;
                OutputConversion::FallibleSerialized(ty)
            }
            OutputConversionDiscriminants::FallibleVoid => FallibleVoid,
        })
    }
}

fn infer_output(input: &ParseStream, jni_type: &Type) -> syn::Result<OutputConversion> {
    let Type::Path(path_ty) = jni_type else {
        return Err(input.error("can't infer return type"))
    };

    let last_segment = extract_last_segment(&path_ty.path);
    Ok(if last_segment.ident == "jstring" {
        OutputConversion::String
    } else if last_segment.ident == "jlong" {
        OutputConversion::I64
    } else if last_segment.ident == "jint" {
        OutputConversion::I32
    } else if last_segment.ident == "jboolean" {
        OutputConversion::Boolean
    } else {
        return Err(input.error("can't infer. You must specify the output conversion"));
    })
}
