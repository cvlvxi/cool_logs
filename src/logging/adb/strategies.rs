use regex::Regex;
use lazy_static::lazy_static;

use crate::logging::{LinePartStrategy, LineParts};

const R_MM_DD: &str = r"(?P<datetime>\d{2}-\d{2})";
const R_HMS: &str = r"(?P<timestamp>\d{2}:\d{2}:\d{2}).\d{3}";
const R_PROCESS: &str = r"\d*";
const R_LOG_LEVEL: &str = r"(?P<loglevel>[A-Za-z]{1})";
const R_PREFIX: &str = r"(?P<prefix>[A-Za-z]+)";
const R_MSG: &str = r"(?P<message>.+)";

struct AdbRegexStrategy {
    pub re_pattern: &'static Regex
}

impl AdbRegexStrategy {
    fn new() -> Self {
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
    type PartType<'a> = LineParts<'a>;
    fn parts<'LLT>(&self, line: &'LLT str) -> Option<Self::PartType<'LLT>> {
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
    println!("{:?}", strategy.parts(some_line));
}



