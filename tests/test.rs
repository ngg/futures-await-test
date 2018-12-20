#![feature(generators)]

extern crate futures_await as futures;
use futures::prelude::r#await;
use futures::prelude::*;
use futures_await_test::async_test;

fn create_future() -> impl Future<Item = u32, Error = ()> {
    Ok(4).into_future()
}

#[async_test(should_panic)]
fn panic_test() -> Result<(), ()> {
    let x = r#await!(create_future())?;
    assert!(x == 5);
    Ok(())
}

#[async_test(should_fail)]
fn fail_test() -> Result<(), ()> {
    let x = r#await!(create_future())?;
    if x == 5 {
        Ok(())
    } else {
        Err(())
    }
}

#[async_test]
fn normal_test() -> Result<(), ()> {
    let x = r#await!(create_future())?;
    assert!(x == 4);
    Ok(())
}
