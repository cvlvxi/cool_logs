
pub mod adb;

use async_trait::async_trait;

use std::ops::Range;

#[derive(Debug)]
pub struct LineParts {
    pub curr_line: String,
    pub datetime: Range<usize>,
    pub timestamp: Range<usize>,
    pub loglevel: Range<usize>,
    pub prefix: Range<usize>,
    pub message: Range<usize>
}


#[async_trait]
pub trait Parser {
    type PartType;
    async fn next(&mut self) -> Option<Self::PartType>;
}

pub trait LinePartStrategy {
    type PartType;
    fn parts(&self, line: String) -> Option<Self::PartType>;
}

