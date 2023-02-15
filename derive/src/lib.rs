#![warn(warnings)]

mod attr;
mod deserialize;
mod serialize;
mod symbol;

#[proc_macro_derive(Deserialize, attributes(envir))]
pub fn deserialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    deserialize::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Serialize, attributes(envir))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    serialize::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub(crate) fn error<R>(ast: &dyn quote::ToTokens, message: &str) -> syn::Result<R> {
    Err(syn::Error::new_spanned(ast, message))
}

pub(crate) fn is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(typepath) = ty else {
        return false
    };

    typepath.path.leading_colon.is_none()
        && typepath.path.segments.len() == 1
        && typepath
            .path
            .segments
            .iter()
            .next()
            .map(|x| x.ident.to_string())
            == Some("Option".to_string())
}
