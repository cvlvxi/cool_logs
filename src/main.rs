#![feature(generic_associated_types)]

mod logging;

use regex::Regex;
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use tokio::time::sleep;

use crate::logging::adb::Adb;
use crate::logging::adb::strategies::AdbRegexStrategy;
use crate::logging::Parser;

async fn sleep_print(msg: &str) {
    println!("{:?}", msg);
    sleep(Duration::from_secs(1)).await;


}

#[tokio::main]
async fn main() {
    let mut adb = Adb::new(AdbRegexStrategy::new());
    
    let a = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            println!("From Logging: {:?}", adb.next().await);

        }
    });

    let b = tokio::spawn(async move {
        loop {
            sleep_print("Hello from other thing").await;
        }
    });

    a.await;


}
