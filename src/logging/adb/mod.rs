pub mod strategies;

use std::process::Stdio;

use crate::logging::LinePartStrategy;

use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::ChildStdout;
use tokio::process::Command;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

use super::Parser;

pub struct Adb<S: LinePartStrategy> {
    strategy: S,
    reader: LinesStream<BufReader<ChildStdout>>,
}

impl<S: LinePartStrategy> Adb<S> {
    pub fn new(strategy: S) -> Self {
        let mut child = Command::new("adb")
            .arg("logcat")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't open adb logcat");

        let stdout = child.stdout.take().expect("Couldn't get stdout");
        let reader = LinesStream::new(BufReader::new(stdout).lines());

        tokio::spawn(async move {
            let status = child
                .wait()
                .await
                .expect("Child process encountered an error");
        });
        Self {
            strategy,
            reader
        }
    }
}

#[async_trait]
impl<S: LinePartStrategy + Send> Parser for Adb<S> {
    type PartType = S::PartType;
    async fn next(&mut self) -> Option<Self::PartType> {
        let line = self.reader.next().await?.unwrap();
        self.strategy.parts(line)
    }
}

#[tokio::test]
async fn create_adb() {
    use crate::logging::adb::strategies::AdbRegexStrategy;

    let mut adb = Adb::new(AdbRegexStrategy::new());
    let some_line = "05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)";

    // println!("{:?}", adb.strategy.parts(some_line));
    // adb.reader.next_
    // loop {

    //     let line= adb.reader.next_line().await;
    //     println!("{:?}", line);
    // }
    // while let Some(something) = adb.reader.next().await {
    //     println!("{:?}", something);
    // }

    loop {
        println!("{:?}", adb.next().await);
    }
}
