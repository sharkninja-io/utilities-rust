use quote::ToTokens;
use syn::parse::ParseStream;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, Type,
};

pub fn extract_single_generic_type<'a>(
    input: &ParseStream,
    arguments: &'a PathArguments,
) -> syn::Result<&'a Type> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
            if args.len() == 1 =>
        {
            if let GenericArgument::Type(ty) = args.first().unwrap() {
                return Ok(ty);
            }
            Err(input.error("single generic type expected"))
        }
        _ => Err(input.error("expected generic")),
    }
}

pub fn extract_last_segment_from_type<'a>(
    input: &ParseStream,
    ty: &'a Type,
) -> syn::Result<&'a PathSegment> {
    Ok(match ty {
        Type::Path(path) => extract_last_segment(&path.path),
        Type::Ptr(ptr) => extract_last_segment_from_type(input, ptr.elem.as_ref())?,
        _ => {
            return Err(input.error(format!(
                "can't extract segment from type: {}",
                ty.to_token_stream()
            )))
        }
    })
}

pub fn extract_last_segment(path: &Path) -> &PathSegment {
    path.segments.last().expect("not a path to type")
}
