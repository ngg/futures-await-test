#![feature(generators, proc_macro)]

extern crate futures_await as futures;
extern crate futures_await_test;
use futures_await_test::async_test;
use futures::prelude::*;

fn create_future() -> impl Future<Item = u32, Error = ()> {
    Ok(4).into_future()
}

#[async_test(should_panic)]
fn panic_test() -> Result<(), ()> {
    let x = await!(create_future())?;
    assert!(x == 5);
    Ok(())
}

#[async_test(should_fail)]
fn fail_test() -> Result<(), ()> {
    let x = await!(create_future())?;
    if x == 5 {
        Ok(())
    } else {
        Err(())
    }
}

#[async_test]
fn normal_test() -> Result<(), ()> {
    let x = await!(create_future())?;
    assert!(x == 4);
    Ok(())
}
