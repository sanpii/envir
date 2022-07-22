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
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::INTERNAL => {
                    param.envir = quote::quote! { crate };
                }
                // Parse #[envir(prefix = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::PREFIX =>
                {
                    let prefix = get_lit_str(crate::symbol::PREFIX, &m.lit)?;
                    param.prefix = Some(prefix);
                }
                syn::NestedMeta::Meta(meta) => {
                    return crate::error(meta.path(), "Unknow elephantry container attribute");
                }
                syn::NestedMeta::Lit(lit) => {
                    return crate::error(lit, "Unexpected literal in elephantry field attribute");
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

#[derive(Clone, Debug)]
pub(crate) enum Default {
    None,
    Trait,
    Path(String),
}

impl std::default::Default for Default {
    fn default() -> Self {
        Self::None
    }
}

impl Field {
    pub fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut param = Self::default();

        for item in flat_map(&field.attrs)? {
            match &item {
                // Parse #[envir(default)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::DEFAULT => {
                    param.default = Default::Trait;
                }
                // Parse #[envir(default = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::DEFAULT =>
                {
                    let value = get_lit_str(crate::symbol::DEFAULT, &m.lit)?;
                    param.default = Default::Path(value);
                }
                // Parse #[envir(export_with = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::EXPORT_WITH =>
                {
                    let export_with = get_lit(crate::symbol::EXPORT_WITH, &m.lit)?;
                    param.export_with = Some(export_with);
                }
                // Parse #[envir(load_with = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m))
                    if m.path == crate::symbol::LOAD_WITH =>
                {
                    let load_with = get_lit(crate::symbol::LOAD_WITH, &m.lit)?;
                    param.load_with = Some(load_with);
                }
                // Parse #[envir(name = "")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(m)) if m.path == crate::symbol::NAME => {
                    let name = get_lit_str(crate::symbol::NAME, &m.lit)?;
                    param.name = Some(name);
                }
                // Parse #[envir(nested)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::NESTED => {
                    param.nested = true;
                }
                // Parse #[envir(noprefix)]
                syn::NestedMeta::Meta(syn::Meta::Path(w)) if w == crate::symbol::NOPREFIX => {
                    param.noprefix = true;
                }
                syn::NestedMeta::Meta(meta) => {
                    return crate::error(meta.path(), "Unknow envir field attribute");
                }
                syn::NestedMeta::Lit(lit) => {
                    return crate::error(lit, "Unexpected literal in envir field attribute");
                }
            }
        }

        Ok(param)
    }
}

fn get_lit(
    attr_name: crate::symbol::Symbol,
    lit: &syn::Lit,
) -> syn::Result<proc_macro2::TokenStream> {
    let lit = get_lit_str(attr_name, lit)?;
    syn::parse_str(&lit)
}

fn get_lit_str(attr_name: crate::symbol::Symbol, lit: &syn::Lit) -> syn::Result<String> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit.value())
    } else {
        crate::error(
            lit,
            &format!(
                "expected elephantry {} attribute to be a string: `{} = \"...\"`",
                attr_name, attr_name
            ),
        )
    }
}

fn flat_map(attrs: &[syn::Attribute]) -> syn::Result<Vec<syn::NestedMeta>> {
    let mut items = Vec::new();

    for attr in attrs {
        items.append(&mut meta_items(attr)?);
    }

    Ok(items)
}

fn meta_items(attr: &syn::Attribute) -> syn::Result<Vec<syn::NestedMeta>> {
    if attr.path != crate::symbol::ENVIR {
        return Ok(Vec::new());
    }

    match attr.parse_meta()? {
        syn::Meta::List(meta) => Ok(meta.nested.into_iter().collect()),
        _ => crate::error(attr, "expected #[envir(...)]"),
    }
}
