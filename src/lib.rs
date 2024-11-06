extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(CStructLayout)]
pub fn derive_c_struct_layout(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let gen = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref _fields) => {
                    // Implementing a basic check for the struct name, can be expanded for detailed checks.
                    quote! {
                        impl #name {
                            pub fn check_layout() {
                                println!("Checking layout for {}", stringify!(#name));
                            }
                        }
                    }
                },
                _ => unimplemented!(),
            }
        },
        _ => unimplemented!(),
    };

    gen.into()
}
