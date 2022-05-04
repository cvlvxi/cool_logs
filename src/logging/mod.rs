use async_trait::async_trait;

#[async_trait]
pub trait Parser {
    // async fn next(&mut self);
}

pub trait LineParts<'LLT> {
    type PartsType<'a>;
    fn parts(&self, line: &'LLT str) -> Option<Self::PartsType<'LLT>>;
}

pub trait PartStrategy<'LLT> {
    type PartsType<'a>;
    fn extract_parts(&self, line: &'LLT str) -> Option<Self::PartsType<'LLT>>;
}

mod adb;
