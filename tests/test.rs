#![feature(async_await, await_macro)]

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
async fn panic_test() {
    let x = await!(create_future());
    assert!(x == 5);
}

#[async_test]
async fn normal_test() {
    let x = await!(create_future());
    assert!(x == 4);
}

#[async_test]
async fn result_test() -> Result<(), ()> {
    let x = await!(create_result_future())?;
    assert!(x == 4);
    Ok(())
}
