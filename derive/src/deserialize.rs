pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let attr = crate::attr::Container::from_ast(ast)?;
    let envir = attr.envir.clone();

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => return crate::error(ast, "this derive macro only works on structs"),
    };

    if matches!(fields, syn::Fields::Unnamed(_)) {
        return crate::error(
            ast,
            "this derive macro only works on structs with named field",
        );
    }

    let from_body = fields
        .iter()
        .map(|x| gen_field(&attr, x))
        .collect::<Result<Vec<_>, _>>()?;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let de = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #envir::Deserialize for #name #ty_generics #where_clause {
            fn from(env: &std::collections::HashMap<String, String>) -> #envir::Result<Self> {
                Ok(Self {
                    #(#from_body, )*
                })
            }
        }
    };

    Ok(de)
}

fn gen_field(
    attr: &crate::attr::Container,
    field: &syn::Field,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_attr = crate::attr::Field::from_ast(field)?;
    let envir = attr.envir.clone();
    let name = &field.ident;
    let mut var = field_attr
        .name
        .clone()
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string().to_uppercase());

    if !field_attr.noprefix {
        var.insert_str(0, attr.prefix.as_deref().unwrap_or(""));
    }

    if let Some(load_with) = field_attr.load_with {
        return Ok(quote::quote! {
            #name: #load_with(&env)?
        });
    }

    if field_attr.nested {
        return Ok(quote::quote! {
            #name: #envir::Deserialize::from(env)?
        });
    }

    if crate::is_option(&field.ty) {
        return Ok(quote::quote! {
            #name: #envir::load_optional_var(env, #var, None)?
        });
    }

    let gen = match &field_attr.default {
        crate::attr::Default::None => quote::quote! {
            #name: #envir::load_optional_var(env, #var, None)?
                .ok_or(#envir::Error::Missing(#var.to_string()))?
        },
        crate::attr::Default::Trait => quote::quote! {
            #name: #envir::load_optional_var(env, #var, None)?
                .unwrap_or_else(::std::default::Default::default)
        },
        crate::attr::Default::Path(path) => quote::quote! {
            #name: #envir::load_optional_var(env, #var, ::std::option::Option::Some(#path.to_string()))?
                .unwrap()
        },
    };

    Ok(gen)
}
