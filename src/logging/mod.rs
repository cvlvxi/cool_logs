
mod adb;

use async_trait::async_trait;

use tokio_stream::wrappers::LinesStream;

#[derive(Debug)]
pub struct LineParts<'LLT> {
    pub datetime: &'LLT str,
    pub timestamp: &'LLT str,
    pub loglevel: &'LLT str,
    pub prefix: &'LLT str,
    pub message: &'LLT str 
}
#[async_trait]
pub trait Parser {
    type PartType<'a>;
    async fn next2<'LLT>(&mut self, line: &'LLT str) -> Option<Self::PartType<'LLT>>;
}

pub trait LinePartStrategy {
    type PartType<'a>;
    fn parts<'LLT>(&self, line: &'LLT str) -> Option<Self::PartType<'LLT>>;
}

