#![feature(proc_macro)]
#![recursion_limit = "4096"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Ident, Item};

#[proc_macro_attribute]
pub fn async_test(params: TokenStream, function: TokenStream) -> TokenStream {
    assert!(
        params.to_string() == "",
        "the #[async_test] attribute currently does not take parameters"
    );

    let mut parsed = syn::parse::<Item>(function).expect("failed to parse tokens as a function");
    let mut attrs = vec![];
    let (ident, inner_ident) = match parsed {
        Item::Fn(ref mut item) => {
            let orig = item.ident;
            let inner_name = "_inner_".to_owned() + orig.as_ref();
            item.ident = Ident::from(inner_name);
            std::mem::swap(&mut attrs, &mut item.attrs);
            (orig, item.ident)
        }
        _ => panic!("#[async_test] can only be applied to functions"),
    };

    quote!(
        #[async]
        #parsed

        #[test]
        #(#attrs )*
        fn #ident() {
            use futures::stable::block_on_stable;
            block_on_stable(#inner_ident()).unwrap();
        }
    ).into()
}
