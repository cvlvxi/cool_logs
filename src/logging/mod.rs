use async_trait::async_trait;

// 05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)
// Date, Time, ProcessId, LogLevel, Prefix, Message

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

pub trait PartStrategy {
    fn extract_parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>>;
}

mod adb;
