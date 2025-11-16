#![warn(warnings)]
#![doc = include_str!("../README.md")]

mod attr;
mod deserialize;
mod serialize;

#[proc_macro_derive(Deserialize, attributes(envir))]
pub fn deserialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input);

    deserialize::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Serialize, attributes(envir))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input);

    serialize::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub(crate) fn error<R>(ast: &dyn quote::ToTokens, message: &str) -> syn::Result<R> {
    Err(syn::Error::new_spanned(ast, message))
}

pub(crate) fn is_option(ty: &syn::Type) -> bool {
    is_ty(ty, "Option")
}

pub(crate) fn is_vec(ty: &syn::Type) -> bool {
    is_ty(ty, "Vec")
}

pub(crate) fn is_option_vec(ty: &syn::Type) -> bool {
    crate::extract_type_from_option(ty)
        .map(crate::is_vec)
        .unwrap_or_default()
}

pub(crate) fn is_ty(ty: &syn::Type, expected: &str) -> bool {
    let syn::Type::Path(typepath) = ty else {
        return false;
    };

    typepath.path.leading_colon.is_none()
        && typepath.path.segments.len() == 1
        && typepath
            .path
            .segments
            .iter()
            .next()
            .map(|x| x.ident.to_string())
            == Some(expected.to_string())
}

// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}
