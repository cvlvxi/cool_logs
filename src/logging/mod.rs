
mod adb;

use async_trait::async_trait;

#[derive(Debug)]
struct LineParts<'LLT> {
    pub datetime: &'LLT str,
    pub timestamp: &'LLT str,
    pub loglevel: &'LLT str,
    pub prefix: &'LLT str,
    pub message: &'LLT str 
}
#[async_trait]
pub trait Parser {
    // async fn next(&mut self);
}

pub trait LineParter  {
    // fn parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>>;
    fn parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>>;
}

pub trait LinePartStrategy {
    fn extract_parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>>;
}

