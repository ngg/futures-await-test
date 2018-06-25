#![feature(async_await, await_macro, futures_api, proc_macro, termination_trait_lib)]

extern crate futures;
extern crate futures_await_test;
use futures::future::lazy;
use futures::prelude::*;
use futures_await_test::async_test;

fn create_future() -> impl Future<Output = u32> {
    lazy(|_| 4)
}

fn create_result_future() -> impl Future<Output = Result<u32, ()>> {
    lazy(|_| Ok(4))
}

#[async_test]
#[should_panic]
fn panic_test() {
    let x = await!(create_future());
    assert!(x == 5);
}

#[async_test]
fn normal_test() {
    let x = await!(create_future());
    assert!(x == 4);
}

#[async_test]
fn result_test() -> Result<(), ()> {
    let x = await!(create_result_future())?;
    assert!(x == 4);
    Ok(())
}
