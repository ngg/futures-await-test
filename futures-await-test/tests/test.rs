#![feature(async_await)]

use futures_await_test::async_test;

async fn create_future() -> u32 {
    4
}

async fn create_result_future() -> Result<u32, ()> {
    Ok(create_future().await)
}

#[async_test]
#[should_panic]
async fn panic_test() {
    assert!(create_future().await == 5);
}

#[async_test]
async fn normal_test() {
    assert!(create_future().await == 4);
}

#[async_test]
async fn result_test() -> Result<(), ()> {
    assert!(create_result_future().await? == 4);
    Ok(())
}
