use proc_macro::TokenStream;
use syn::{ItemStruct, parse::Parser, parse_macro_input};

#[proc_macro_derive(Entity)]
pub fn entity_derive_macro(input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let identifier = item_struct.ident.clone();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote::quote! { pub teste: u32 })
                .unwrap(),
        );
    }

    quote::quote! {
        #item_struct
        //
        // impl Entity for #identifier {
        //     fn get_id(&self) -> u32 {
        //         self.teste
        //     }
        // }
    }
    .into()
}
