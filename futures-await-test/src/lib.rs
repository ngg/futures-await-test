#[doc(inline)]
pub use futures_await_test_macro::async_test;

#[doc(hidden)]
pub mod reexport {
    #[doc(hidden)]
    pub use futures_executor::LocalPool;
}
