use crate::parsing::utils::extract_last_segment_from_type;
use proc_macro2::Ident;
use strum_macros::{EnumDiscriminants, EnumString};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Token, Type};

#[derive(EnumDiscriminants)]
#[strum_discriminants(vis(pub(self)), derive(EnumString))]
pub enum ArgConversion {
    U8,
    I32,
    U32,
    I64,
    Double,
    Boolean,
    String,
    Bytes,
    StringList,
    Serialized(Type),
    OptionalI32,
    OptionalU32,
    OptionalBool,
    OptionalString,
    OptionalSerialized(Type),
}

pub struct BindingArgs {
    pub jni_env: Type,
    pub jclass: Option<Type>,
    pub rest: Vec<Type>,
    pub conversions: Vec<ArgConversion>,
}

struct MacroArg {
    jni_type: Type,
    conversion: Option<ArgConversion>,
}

pub fn parse_binding_args(input: &ParseStream) -> syn::Result<BindingArgs> {
    let content;
    parenthesized!(content in input);
    let args: Punctuated<MacroArg, Token![,]> =
        content.parse_terminated(MacroArg::parse, Token![,])?;
    let (jni_env, jclass, rest) = parse_jni_args(input, args)?;

    let rest = rest
        .into_iter()
        .map(|arg| {
            if let Some(conv) = arg.conversion {
                Ok((arg.jni_type, conv))
            } else {
                infer_conversion(input, &arg.jni_type).map(|conv| (arg.jni_type, conv))
            }
        })
        .collect::<syn::Result<Vec<(Type, ArgConversion)>>>()?;
    let (rest, conversions) = rest.into_iter().unzip();
    Ok(BindingArgs {
        jni_env,
        jclass,
        rest,
        conversions,
    })
}

impl Parse for ArgConversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let variant: ArgConversionDiscriminants = name
            .to_string()
            .parse()
            .map_err(|_| input.error("unsupported arg conversion"))?;
        Ok(match variant {
            ArgConversionDiscriminants::U8 => ArgConversion::U8,
            ArgConversionDiscriminants::I32 => ArgConversion::I32,
            ArgConversionDiscriminants::U32 => ArgConversion::U32,
            ArgConversionDiscriminants::I64 => ArgConversion::I64,
            ArgConversionDiscriminants::Double => ArgConversion::Double,
            ArgConversionDiscriminants::Boolean => ArgConversion::Boolean,
            ArgConversionDiscriminants::String => ArgConversion::String,
            ArgConversionDiscriminants::Bytes => ArgConversion::Bytes,
            ArgConversionDiscriminants::StringList => ArgConversion::StringList,
            ArgConversionDiscriminants::Serialized => {
                ArgConversion::Serialized(parse_parenthesized_type(&input)?)
            }
            ArgConversionDiscriminants::OptionalU32 => ArgConversion::OptionalU32,
            ArgConversionDiscriminants::OptionalI32 => ArgConversion::OptionalI32,
            ArgConversionDiscriminants::OptionalBool => ArgConversion::OptionalBool,
            ArgConversionDiscriminants::OptionalString => ArgConversion::OptionalString,
            ArgConversionDiscriminants::OptionalSerialized => {
                ArgConversion::OptionalSerialized(parse_parenthesized_type(&input)?)
            }
        })
    }
}

impl Parse for MacroArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let jni_type: Type = input.parse()?;
        let conversion: Option<ArgConversion> = if input.peek(Token![=]) {
            let _paren: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(MacroArg {
            jni_type,
            conversion,
        })
    }
}

// Returns jnienv, jclass and the rest.
fn parse_jni_args(
    input: &ParseStream,
    args: Punctuated<MacroArg, Token![,]>,
) -> syn::Result<(Type, Option<Type>, Vec<MacroArg>)> {
    let mut iter = args.into_iter().peekable();
    let jni_env = iter
        .next()
        .ok_or(input.error("JNIEnv expected as the first arg"))?;
    let next_item = iter.peek();
    let mut has_jclass = false;
    if let Some(arg) = next_item {
        let last_segment = extract_last_segment_from_type(input, &arg.jni_type)?;
        if last_segment.ident == "JClass" {
            has_jclass = true;
        }
    }
    let jclass = if has_jclass { iter.next() } else { None };

    let rest: Vec<MacroArg> = iter.collect();
    Ok((jni_env.jni_type, jclass.map(|arg| arg.jni_type), rest))
}

fn infer_conversion(input: &ParseStream, jni_type: &Type) -> syn::Result<ArgConversion> {
    let last_segment = extract_last_segment_from_type(input, jni_type)?;
    Ok(if last_segment.ident == "jboolean" {
        ArgConversion::Boolean
    } else if last_segment.ident == "JString" {
        ArgConversion::String
    } else if last_segment.ident == "jlong" {
        ArgConversion::I64
    } else if last_segment.ident == "jint" {
        ArgConversion::I32
    } else if last_segment.ident == "jdouble" {
        ArgConversion::Double
    } else if last_segment.ident == "JObject" {
        return Err(input.error("JObject conversion can't be inferred"));
    } else {
        return Err(input.error("can't infer arg conversion"));
    })
}

fn parse_parenthesized_type(input: &ParseStream) -> syn::Result<Type> {
    let content;
    parenthesized!(content in input);
    content.parse()
}
