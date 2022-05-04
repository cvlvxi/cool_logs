use crate::logging::{Parser, LineParts, PartStrategy};

use std::marker::PhantomData;

use async_trait::async_trait;
use regex::Regex;

// LLT = LineLifeTime

struct Adb<'LLT, S: PartStrategy<'LLT>> {
    strategy: S,
    phantom: PhantomData<&'LLT ()>
}

impl<'LLT, S: PartStrategy<'LLT>> Adb<'LLT, S> {
    fn new(strategy: S) -> Self {
        Self {
            strategy,
            phantom: PhantomData{}
        }
    }
}

// #[async_trait]
// impl<'LLT, S:PartStrategy<'LLT>> Parser for Adb<'LLT, S> {
//     async fn next(&mut self) -> Option<AdbParts> {
//         None
//     }
// }

impl<'LLT, S: PartStrategy<'LLT>> LineParts<'LLT> for Adb<'LLT, S> {
    type PartsType<'a> = AdbParts<'LLT>;
    fn parts(&self, line: &str) -> Option<AdbParts<'LLT>>{
        return self.strategy.extract_parts(line)
    }
}



struct AdbRegexStrategy {
    pub re_pattern: Regex
}

impl AdbRegexStrategy {
    fn new() -> Self {
        //// 05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)
        let re_joiner = r"\s+";
        let re_datetime =  r"(?P<datetime>\d{2}-\d{2})";
        let re_timestamp = r"(?P<timestamp>\d{2}:\d{2}:\d{2}.\d{3})";
        let re_process = r"\d+";
        let re_loglevel = r"(?P<loglevel>[A-Za-z])";
        let re_prefix = r"(?P<prefix>[A-Za-z0-9]+)";
        let re_message = r"(?P<message>.+)";

        let re_pattern = [re_datetime, re_timestamp, re_process, re_process, re_loglevel, re_prefix, re_message].join(re_joiner);

        Self {
            re_pattern: Regex::new(&re_pattern).unwrap()
        }
    }
}

impl<'LLT> PartStrategy<'LLT> for AdbRegexStrategy {
    type PartsType<'a> = AdbParts<'LLT>;
    fn extract_parts(&self, line: &'LLT str) -> Option<Self::PartsType<'LLT>> {
        let capture = self.re_pattern.captures(line)?;
        Some(AdbParts{
            datetime: &capture["datetime"],
            timestamp: &capture["timestamp"],
            loglevel: &capture["loglevel"],
            prefix: &capture["prefix"],
            message: &capture["message"]
        })
    }
}


// 05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)
// Date, Time, ProcessId, LogLevel, Prefix, Message 
struct AdbParts<'LLT> {
    pub datetime: &'LLT str,
    pub timestamp: &'LLT str,
    pub loglevel: &'LLT str,
    pub prefix: &'LLT str,
    pub message: &'LLT str 
}

#[test]
fn test_regex() {
    let some_line = "05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)";
    let strategy = AdbRegexStrategy::new();
    println!("{:?}", strategy.extract_parts(some_line));

    
}




