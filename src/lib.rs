#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Ident, Item};

#[proc_macro_attribute]
pub fn async_test(attribute: TokenStream, function: TokenStream) -> TokenStream {
    let mut parsed = syn::parse::<Item>(function).expect("failed to parse tokens as a function");
    let (ident, inner_ident) = match parsed {
        Item::Fn(ref mut item) => {
            let orig = item.ident;
            let inner_name = "_inner_".to_owned() + orig.as_ref();
            item.ident = Ident::new(&inner_name, orig.span);
            (orig, item.ident)
        }
        _ => panic!("#[async_test] can only be applied to functions"),
    };

    let attribute = attribute.to_string();
    let call = if attribute == "( should_panic )" {
        quote_spanned!(Span::call_site() =>
            use std;
            let result = std::panic::catch_unwind(|| #inner_ident().wait());
            if result.is_ok() {
                panic!("test did not panic");
            }
        )
    } else if attribute == "( should_fail )" {
        quote_spanned!(Span::call_site() =>
            let result = #inner_ident().wait();
            if result.is_ok() {
                panic!("test did not fail")
            }
        )
    } else if attribute == "" {
        quote_spanned!(Span::call_site() =>
            #inner_ident().wait().unwrap();
        )
    } else {
        panic!("the #[async_test] attribute currently only takes `should_panic` or `should_fail` as an arg");
    };
    let output = quote_spanned!(Span::call_site() =>
        #[async]
        #parsed

        #[test]
        fn #ident() {
            #call
        }
    );
    output.into()
}
