use async_trait::async_trait;

#[async_trait]
pub trait Dog {
    async fn doSomething();
}

struct SomeStruct {}

#[async_trait]
impl Dog for SomeStruct {
    async fn doSomething() {

    }
}

#[tokio::main]
async fn main() {

}