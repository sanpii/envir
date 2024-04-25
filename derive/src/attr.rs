#[derive(Clone, Debug, darling::FromDeriveInput)]
#[darling(attributes(envir), supports(struct_named))]
pub(crate) struct Container {
    #[darling(default)]
    pub internal: bool,
    pub prefix: Option<String>,
}

impl Container {
    pub fn envir(&self) -> proc_macro2::TokenStream {
        if self.internal {
            quote::quote! { crate }
        } else {
            quote::quote! { envir }
        }
    }
}

#[derive(Clone, Default, Debug, darling::FromField)]
#[darling(attributes(envir))]
pub(crate) struct Field {
    #[darling(default)]
    pub default: Option<darling::util::Override<String>>,
    pub export_with: Option<syn::Ident>,
    pub load_with: Option<syn::Ident>,
    pub name: Option<String>,
    #[darling(default)]
    pub noprefix: bool,
    #[darling(default)]
    pub nested: bool,
}
