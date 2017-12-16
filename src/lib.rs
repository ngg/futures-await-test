#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro2;
extern crate proc_macro;
#[macro_use]
extern crate futures_await_quote as quote;
extern crate futures_await_syn as syn;

use proc_macro2::Term;
use proc_macro::TokenStream;
use syn::{Ident, Item, ItemKind};

#[proc_macro_attribute]
pub fn async_test(attribute: TokenStream, function: TokenStream) -> TokenStream {
    let mut parsed = syn::parse::<Item>(function).expect("failed to parse tokens as a function");
    let (ident, inner_ident) = match parsed.node {
        ItemKind::Fn(ref mut item) => {
            let orig = item.ident;
            let inner_name = "_inner_".to_owned() + orig.sym.as_str();
            item.ident = Ident::new(Term::intern(&inner_name), orig.span);
            (orig, item.ident)
        },
        _ => panic!("#[async_test] can only be applied to functions"),
    };

    let attribute = attribute.to_string();
    let call = if attribute == "( should_panic )" {
        quote!(
            use std;
            let result = std::panic::catch_unwind(|| #inner_ident().wait());
            if result.is_ok() {
                panic!("test did not panic");
            }
        )
    } else if attribute == "( should_fail )" {
        quote!(
            let result = #inner_ident().wait();
            if result.is_ok() {
                panic!("test did not fail")
            }
        )
    } else if attribute == "" {
        quote!(
            #inner_ident().wait().unwrap();
        )
    } else {
        panic!("the #[async_test] attribute currently only takes `should_panic` or `should_fail` as an arg");
    };
    let output = quote!(
        #[async]
        #parsed

        #[test]
        fn #ident() {
            #call
        }
    );
    output.into()
}
