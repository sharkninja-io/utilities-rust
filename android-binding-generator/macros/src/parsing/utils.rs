use quote::ToTokens;
use syn::parse::ParseStream;
use syn::{Path, PathSegment, Type};

pub fn extract_last_segment_from_type<'a>(
    input: &ParseStream,
    ty: &'a Type,
) -> syn::Result<&'a PathSegment> {
    Ok(match ty {
        Type::Path(path) => extract_last_segment(&path.path),
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
