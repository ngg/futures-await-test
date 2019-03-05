#![recursion_limit = "4096"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Ident, Item, Visibility};

#[proc_macro_attribute]
pub fn async_test(params: TokenStream, input: TokenStream) -> TokenStream {
    assert!(
        params.is_empty(),
        "the #[async_test] attribute currently does not take parameters"
    );

    let mut inner = parse_macro_input!(input as Item);
    let mut outer = inner.clone();
    if let (&mut Item::Fn(ref mut inner_fn), &mut Item::Fn(ref mut outer_fn)) =
        (&mut inner, &mut outer)
    {
        inner_fn.ident = Ident::new(
            &("_inner_".to_string() + &inner_fn.ident.to_string()),
            Span::call_site(),
        );
        let inner_ident = &inner_fn.ident;
        inner_fn.vis = Visibility::Inherited;
        inner_fn.attrs.clear();
        assert!(
            outer_fn.asyncness.take().is_some(),
            "#[async_test] can only be applied to async functions"
        );
        outer_fn.attrs.push(parse_quote!(#[test]));
        outer_fn.block = Box::new(parse_quote!({
            ::futures::executor::LocalPool::new().run_until(#inner_ident())
        }));
    } else {
        panic!("#[async_test] can only be applied to async functions")
    }
    quote!(
        #inner
        #outer
    )
    .into()
}
