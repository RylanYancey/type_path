use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DeriveInput, parse_macro_input};
use syn::{GenericParam, LitStr, parse_quote};

#[proc_macro_derive(TypePath)]
pub fn derive_type_path(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let attrs = get_attrs(&input).expect("Failed to parse TypePath derive attributes.");

    let path = attrs
        .path
        .unwrap_or_else(|| LitStr::new(module_path!(), Span::call_site()));
    let name = attrs
        .name
        .unwrap_or_else(|| LitStr::new(&ident.to_string(), Span::call_site()));

    if input.generics.params.is_empty() {
        let ident = &input.ident;
        quote! {
            impl TypePath for #ident {
                fn type_path() -> &'static str {
                    concat!(#path, "::", #name)
                }
            }
        }
        .into()
    } else {
        impl_type_path_with_generics(path, name, &mut input)
    }
}

fn impl_type_path_with_generics(
    path: LitStr,
    name: LitStr,
    input: &mut DeriveInput,
) -> TokenStream {
    let params = input.generics.params.iter().map(|param| match param {
        GenericParam::Type(ty) => {
            let ident = &ty.ident;
            quote!(<#ident as TypePath>::type_path())
        }
        GenericParam::Lifetime(_) => quote!("'_"),
        GenericParam::Const(c) => {
            let ident = &c.ident;
            quote!(stringify!(#ident))
        }
    });

    let concat_generics = quote! {
        let mut result = String::from(concat!(#path, "::", #name, "<"));
        #(result.push_str(#params); result.push(','))*
        result.push('>');
        result
    };

    // Push a TypePath bound to each generic parameter.
    input.generics.params.iter_mut().for_each(|param| {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(parse_quote!(TypePath))
        }
    });

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;
    quote! {
        impl #impl_generics TypePath for #ident #type_generics #where_clause {
            fn type_path() -> &'static str {
                use ::type_path::cell::GenericTypeCell;
                static CELL: GenericTypeCell = GenericTypeCell::new();
                CELL.get_or_insert::<Self, _>(|| { #concat_generics })
            }
        }
    }
    .into()
}

fn get_attrs(input: &DeriveInput) -> syn::Result<TypePathAttrs> {
    let mut attrs = TypePathAttrs::default();

    for attr in &input.attrs {
        if attr.path().is_ident("type_path") {
            match attr.parse_args::<LitStr>() {
                Ok(s) => {
                    if attrs.path.is_some() {
                        return Err(syn::Error::new(
                            attr.span(),
                            "duplicate type_path definition.",
                        ));
                    }
                    attrs.path = Some(s);
                }
                Err(_) => {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            if attrs.name.is_some() {
                                return Err(
                                    meta.error("duplicate `name` key in type_path attribute.")
                                );
                            }
                            let value: LitStr = meta.value()?.parse()?;
                            attrs.name = Some(value);
                        } else if meta.path.is_ident("path") {
                            if attrs.path.is_some() {
                                return Err(
                                    meta.error("duplicate `path` key in type_path attribute.")
                                );
                            }
                            let value: LitStr = meta.value()?.parse()?;
                            attrs.path = Some(value);
                        } else {
                            return Err(meta.error("Unknown key, expected `name` or `path`"));
                        }

                        Ok(())
                    })?;
                }
            }
        } else if attr.path().is_ident("type_name") {
            if attrs.name.is_some() {
                return Err(syn::Error::new(
                    attr.span(),
                    "duplicate type_name definition.",
                ));
            }
            attrs.name = Some(attr.parse_args::<LitStr>()?);
        }
    }

    Ok(attrs)
}

#[derive(Default)]
struct TypePathAttrs {
    name: Option<LitStr>,
    path: Option<LitStr>,
}
