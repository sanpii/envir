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

    let mut export_body = Vec::new();

    for field in fields {
        export_body.push(gen_field(&attr, field)?);
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let de = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #envir::Serialize for #name #ty_generics #where_clause {
            fn collect(&self) -> std::collections::HashMap<String, String> {
                let mut hash_map = std::collections::HashMap::new();

                #(#export_body; )*

                hash_map
            }
        }
    };

    Ok(de)
}

fn gen_field(
    attr: &crate::attr::Container,
    field: &syn::Field,
) -> syn::Result<proc_macro2::TokenStream> {
    let envir = attr.envir.clone();
    let field_attr = crate::attr::Field::from_ast(field)?;
    let name = &field.ident;
    let var = format!(
        "{}{}",
        attr.prefix.as_deref().unwrap_or(""),
        field_attr
            .name
            .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string().to_uppercase())
    );

    if let Some(export_with) = field_attr.export_with {
        return Ok(quote::quote! {
            hash_map.extend(#export_with(&self.#name));
        });
    }

    let gen = if crate::is_option(&field.ty) && field_attr.nested {
        quote::quote! {
            if let Some(ref v) = self.#name {
                hash_map.extend(#envir::Serialize::collect(v));
            }
        }
    } else if crate::is_option(&field.ty) {
        quote::quote! {
            if let Some(ref v) = self.#name {
                hash_map.insert(#var.to_string(), v.to_string());
            }
        }
    } else if field_attr.nested {
        quote::quote! {
            hash_map.extend(#envir::Serialize::collect(&self.#name))
        }
    } else {
        quote::quote! {
            hash_map.insert(#var.to_string(), self.#name.to_string())
        }
    };

    Ok(gen)
}
