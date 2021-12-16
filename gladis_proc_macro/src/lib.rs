// SPDX-FileCopyrightText: 2021 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

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
            fn from_builder(builder: gtk::Builder) -> gladis::Result<Self> {
                Ok(Self {
                    #(
                        #field_name: builder.object(stringify!(#field_name))
                            .ok_or(gladis::GladisError::not_found(
                                stringify!(#field_name),
                                stringify!(#field_type),
                            ))?,
                    )*
                })
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Gladis)]
pub fn derive_gladis(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gladis(&ast)
}
