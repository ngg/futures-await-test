#![recursion_limit = "4096"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote_spanned;
use syn::{Ident, Item};

#[proc_macro_attribute]
pub fn async_test(attribute: TokenStream, function: TokenStream) -> TokenStream {
    let mut parsed = syn::parse::<Item>(function).expect("failed to parse tokens as a function");
    let (ident, inner_ident) = match parsed {
        Item::Fn(ref mut item) => {
            let orig = item.ident.clone();
            let inner_name = "_inner_".to_owned() + &orig.to_string();
            item.ident = Ident::new(&inner_name, Span::call_site());
            (orig, item.ident.clone())
        }
        _ => panic!("#[async_test] can only be applied to functions"),
    };

    let span = Span::call_site();
    let call = match attribute.to_string().as_ref() {
        "should_panic" => quote_spanned!(span=>
            use std;
            let result = std::panic::catch_unwind(|| #inner_ident().wait());
            if result.is_ok() {
                panic!("test did not panic");
            }
        ),
        "should_fail" => quote_spanned!(span=>
            let result = #inner_ident().wait();
            if result.is_ok() {
                panic!("test did not fail")
            }
        ),
        "" => quote_spanned!(span=>
            #inner_ident().wait().unwrap();
        ),
        _ => panic!("the #[async_test] attribute currently only takes `should_panic` or `should_fail` as an arg")
    };

    quote_spanned!(span=>
        #[r#async]
        #parsed

        #[test]
        fn #ident() {
            #call
        }
    ).into()
}
