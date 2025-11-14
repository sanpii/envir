pub(crate) fn impl_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    use darling::FromDeriveInput;

    let attr = crate::attr::Container::from_derive_input(ast).unwrap();
    let envir = attr.envir();

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
    use darling::FromField;

    let field_attr = crate::attr::Field::from_field(field)?;
    let envir = attr.envir();
    let name = &field.ident;
    let mut var = field_attr
        .name
        .clone()
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string().to_uppercase());

    if !field_attr.noprefix {
        var.insert_str(0, attr.prefix.as_deref().unwrap_or(""));
    }

    if field_attr.skip || field_attr.skip_load {
        return Ok(quote::quote! {
            #name: Default::default()
        });
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

    let load = if crate::is_vec(&field.ty) {
        quote::quote! { load_vec }
    } else {
        quote::quote! { load_optional_var }
    };

    let separator = field_attr.separator.unwrap_or(',');

    if crate::is_option(&field.ty) {
        return Ok(quote::quote! {
            #name: #envir::#load(env, #var, None, #separator)?
        });
    }

    let r#gen = match &field_attr.default {
        None => quote::quote! {
            #name: #envir::#load(env, #var, None, #separator)?
                .ok_or(#envir::Error::Missing(#var.to_string()))?
        },
        Some(darling::util::Override::Inherit) => quote::quote! {
            #name: #envir::#load(env, #var, None, #separator)?
                .unwrap_or_else(::std::default::Default::default)
        },
        Some(darling::util::Override::Explicit(path)) => quote::quote! {
            #name: #envir::#load(env, #var, ::std::option::Option::Some(#path.to_string()), #separator)?
                .unwrap()
        },
    };

    Ok(r#gen)
}
