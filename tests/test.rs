#![feature(generators, pin)]

extern crate futures;
extern crate futures_await_test;
use futures::prelude::*;
use futures::prelude::await;
use futures_await_test::async_test;

fn create_future() -> impl Future<Item = u32, Error = ()> {
    Ok(4).into_future()
}

#[async_test]
#[should_panic]
fn panic_test() -> Result<(), ()> {
    let x = await!(create_future())?;
    assert!(x == 5);
    Ok(())
}

#[async_test]
fn normal_test() -> Result<(), ()> {
    let x = await!(create_future())?;
    assert!(x == 4);
    Ok(())
}
