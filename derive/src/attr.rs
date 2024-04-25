#[derive(Clone, Debug)]
pub(crate) struct Container {
    pub envir: proc_macro2::TokenStream,
    pub prefix: Option<String>,
}

impl std::default::Default for Container {
    fn default() -> Self {
        Self {
            envir: quote::quote! { envir },
            prefix: None,
        }
    }
}

impl Container {
    pub fn from_ast(ast: &syn::DeriveInput) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&ast.attrs)? {
            match &item {
                // Parse #[envir(internal)]
                syn::Meta::Path(w) if w == crate::symbol::INTERNAL => {
                    param.envir = quote::quote! { crate };
                }
                // Parse #[envir(prefix = "")]
                syn::Meta::NameValue(m) if m.path == crate::symbol::PREFIX => {
                    let prefix = get_lit_str(crate::symbol::PREFIX, &m.value)?;
                    param.prefix = Some(prefix);
                }
                _ => {
                    return crate::error(item.path(), "Unknow envir container attribute");
                }
            }
        }

        Ok(param)
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Field {
    pub default: Default,
    pub export_with: Option<proc_macro2::TokenStream>,
    pub load_with: Option<proc_macro2::TokenStream>,
    pub name: Option<String>,
    pub noprefix: bool,
    pub nested: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) enum Default {
    #[default]
    None,
    Trait,
    Path(String),
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&field.attrs)? {
            match &item {
                // Parse #[envir(default)]
                syn::Meta::Path(w) if w == crate::symbol::DEFAULT => {
                    param.default = Default::Trait;
                }
                // Parse #[envir(default = "")]
                syn::Meta::NameValue(m) if m.path == crate::symbol::DEFAULT => {
                    let value = get_lit_str(crate::symbol::DEFAULT, &m.value)?;
                    param.default = Default::Path(value);
                }
                // Parse #[envir(export_with = "")]
                syn::Meta::NameValue(m) if m.path == crate::symbol::EXPORT_WITH => {
                    let export_with = get_lit(crate::symbol::EXPORT_WITH, &m.value)?;
                    param.export_with = Some(export_with);
                }
                // Parse #[envir(load_with = "")]
                syn::Meta::NameValue(m) if m.path == crate::symbol::LOAD_WITH => {
                    let load_with = get_lit(crate::symbol::LOAD_WITH, &m.value)?;
                    param.load_with = Some(load_with);
                }
                // Parse #[envir(name = "")]
                syn::Meta::NameValue(m) if m.path == crate::symbol::NAME => {
                    let name = get_lit_str(crate::symbol::NAME, &m.value)?;
                    param.name = Some(name);
                }
                // Parse #[envir(nested)]
                syn::Meta::Path(w) if w == crate::symbol::NESTED => {
                    param.nested = true;
                }
                // Parse #[envir(noprefix)]
                syn::Meta::Path(w) if w == crate::symbol::NOPREFIX => {
                    param.noprefix = true;
                }
                _ => {
                    return crate::error(item.path(), "Unknow envir field attribute");
                }
            }
        }

        Ok(param)
    }
}

fn get_lit(
    attr_name: crate::symbol::Symbol,
    value: &syn::Expr,
) -> syn::Result<proc_macro2::TokenStream> {
    let lit = get_lit_str(attr_name, value)?;
    syn::parse_str(&lit)
}

fn get_lit_str(attr_name: crate::symbol::Symbol, value: &syn::Expr) -> syn::Result<String> {
    if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = value {
        if let syn::Lit::Str(lit) = lit {
            return Ok(lit.value());
        }
    }

    crate::error(
        value,
        &format!("expected envir {attr_name} attribute to be a string: `{attr_name} = \"...\"`"),
    )
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<syn::Meta>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.extend(meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(
    attr: &syn::Attribute,
) -> syn::Result<syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>> {
    if attr.path() != crate::symbol::ENVIR {
        return Ok(syn::punctuated::Punctuated::default());
    }

    match &attr.meta {
        syn::Meta::List(meta) => {
            meta.parse_args_with(syn::punctuated::Punctuated::parse_terminated)
        }
        _ => crate::error(attr, "expected #[envir(...)]"),
    }
}
