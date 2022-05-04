use crate::logging::{Parser, LineParts, LineParter, PartStrategy};

use std::marker::PhantomData;

use async_trait::async_trait;
use regex::Regex;


struct Adb<S: PartStrategy> {
    strategy: S
}

impl<S: PartStrategy> Adb<S> {
    fn new(strategy: S) -> Self {
        Self {
            strategy
        }
    } 
}

// Adb::<AdbRegexStrategy>::new();


// #[async_trait]
// impl<'LLT, S:PartStrategy<'LLT>> Parser for Adb<'LLT, S> {
//     async fn next(&mut self) -> Option<AdbParts> {
//         None
//     }
// }

impl<S: PartStrategy> LineParter for Adb<S> {
    fn parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>>{
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

impl PartStrategy for AdbRegexStrategy {
    fn extract_parts<'LLT>(&self, line: &'LLT str) -> Option<LineParts<'LLT>> {
        let capture = self.re_pattern.captures(&line)?;
        Some(LineParts{
            datetime: &capture.name("datetime").unwrap().as_str(),
            timestamp: &capture.name("timestamp").unwrap().as_str(),
            loglevel: &capture.name("loglevel").unwrap().as_str(),
            prefix: &capture.name("prefix").unwrap().as_str(),
            message: &capture.name("message").unwrap().as_str()
        })
    }
}



#[test]
fn test_regex() {
    let some_line = "05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)";
    let strategy = AdbRegexStrategy::new();
    println!("{:?}", strategy.extract_parts(some_line));

    
}




