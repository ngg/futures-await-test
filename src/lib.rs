#![recursion_limit = "4096"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
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
            let orig = item.ident.clone();
            let inner_name = "_inner_".to_owned() + &orig.to_string();
            item.ident = Ident::new(&inner_name, Span::call_site());
            std::mem::swap(&mut attrs, &mut item.attrs);
            (orig, item.ident.clone())
        }
        _ => panic!("#[async_test] can only be applied to functions"),
    };

    let ret_type = if attrs.iter().any(|a| {
        a.path
            .segments
            .first()
            .map(|s| s.into_value().ident == "should_panic")
            .unwrap_or(false)
    }) {
        quote!(())
    } else {
        quote!(impl ::std::process::Termination)
    };

    quote!(
        async #parsed

        #[test]
        #(#attrs )*
        fn #ident() -> #ret_type {
            use ::futures::executor::LocalPool;
            let mut pool = LocalPool::new();
            let mut spawn = pool.spawner();
            pool.run_until(#inner_ident(), &mut spawn)
        }
    ).into()
}
