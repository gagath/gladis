//! A Rust crate to easily import Glade-generated UI files into Rust code (proc
//! macros).

use proc_macro::TokenStream;
use quote::quote;

use syn::{Data, DataStruct, DeriveInput, Fields};

fn impl_gladis(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    let gen = quote! {
        impl gladis::Gladis for #name {
            fn from_builder(builder: gtk::Builder) -> Self {
                Self {
                    #(
                        #field_name: builder.get_object(stringify!(#field_name))
                            .expect(concat!("could not find \"",
                                            stringify!(#field_name),
                                            "\" of type \"",
                                            stringify!(#field_type),
                                            "\" in glade file")),
                    )*
                }
            }
        }
    };
    gen.into()
}

/// Automatically implement Gladis trait for a struct
///
/// ```
/// use gtk::prelude::*;
/// use gladis::Gladis;
///
/// #[derive(Gladis, Clone)]
/// pub struct Window {
///     pub window: gtk::ApplicationWindow,
///     pub label: gtk::Label,
/// }
/// ```
#[proc_macro_derive(Gladis)]
pub fn derive_gladis(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gladis(&ast)
}
