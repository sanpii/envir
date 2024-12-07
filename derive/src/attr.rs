#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(envir), supports(struct_named))]
pub(crate) struct Container {
    pub prefix: Option<String>,
}

impl Container {
    pub fn envir(&self) -> proc_macro2::TokenStream {
        match (
            proc_macro_crate::crate_name("envir"),
            std::env::var("CARGO_CRATE_NAME").as_deref(),
        ) {
            (Ok(proc_macro_crate::FoundCrate::Itself), Ok("envir")) => quote::quote!(crate),
            (Ok(proc_macro_crate::FoundCrate::Name(name)), _) => {
                let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
                quote::quote!(::#ident)
            }
            _ => quote::quote!(::envir),
        }
    }
}

#[derive(Clone, Default, Debug, darling::FromField)]
#[darling(attributes(envir))]
pub(crate) struct Field {
    #[darling(default)]
    pub default: Option<darling::util::Override<String>>,
    pub export_with: Option<syn::ExprPath>,
    pub load_with: Option<syn::ExprPath>,
    pub name: Option<String>,
    #[darling(default)]
    pub noprefix: bool,
    #[darling(default)]
    pub nested: bool,
    #[darling(default)]
    pub skip_export: bool,
    #[darling(default)]
    pub skip_load: bool,
    #[darling(default)]
    pub skip: bool,
    pub skip_export_if: Option<syn::ExprPath>,
}
