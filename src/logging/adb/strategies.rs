use regex::Regex;
use lazy_static::lazy_static;

use crate::logging::{LinePartStrategy, LineParts};

const R_MM_DD: &str = r"(?P<datetime>\d{2}-\d{2})";
const R_HMS: &str = r"(?P<timestamp>\d{2}:\d{2}:\d{2}).\d{3}";
const R_PROCESS: &str = r"\d*";
const R_LOG_LEVEL: &str = r"(?P<loglevel>[A-Za-z]{1})";
const R_PREFIX: &str = r"(?P<prefix>[A-Za-z]+)";
const R_MSG: &str = r"(?P<message>.+)";

pub struct AdbRegexStrategy {
    pub re_pattern: &'static Regex
}

impl AdbRegexStrategy {
    pub fn new() -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                format!(
                    r"{}\s*{}\s*{}\s*{}\s*{}\s*{}\s*:\s*{}",
                    R_MM_DD, R_HMS, R_PROCESS, R_PROCESS, R_LOG_LEVEL, R_PREFIX, R_MSG
                )
                .as_str()
            )
            .unwrap();
        }
        Self { re_pattern: &RE }
    }
}

impl LinePartStrategy for AdbRegexStrategy {
    type PartType = LineParts;
    fn parts(&self, line: String) -> Option<Self::PartType> {
        let capture = self.re_pattern.captures(&line)?;
        let mut parts = LineParts{
            curr_line: String::from(""),
            datetime: capture.name("datetime").unwrap().range(),
            timestamp: capture.name("timestamp").unwrap().range(),
            loglevel: capture.name("loglevel").unwrap().range(),
            prefix: capture.name("prefix").unwrap().range(),
            message: capture.name("message").unwrap().range()
        };
        parts.curr_line = line; 
        Some(parts)
    }
}



#[test]
fn test_regex() {
    let some_line = "05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)";
    let strategy = AdbRegexStrategy::new();
    let parts = strategy.parts(String::from(some_line)).unwrap();
    println!("datetime: {:?}", &parts.curr_line[parts.datetime]);
    println!("timestamp: {:?}", &parts.curr_line[parts.timestamp]);
    println!("message: {:?}", &parts.curr_line[parts.message]);
}



