use syn::{parse_macro_input, parse::{self, Parse, ParseStream}};
use quote::{ToTokens, quote};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Record, attributes(custom_path))]
pub fn example(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::ItemStruct);
    let retval = handle(parsed);
    retval.to_token_stream().into()
}

fn handle(thing: syn::ItemStruct) -> TokenStream2 {
  let path = thing.ident;
  quote! {
    impl Record for #path {
      fn path() -> &'static str{
        #path
      }
    }
  }
} 