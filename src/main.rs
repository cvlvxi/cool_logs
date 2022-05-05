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
use crate::logging::LineParts;

use tokio::sync::mpsc;

async fn sleep_print(msg: &str) {
    println!("{:?}", msg);
    sleep(Duration::from_secs(1)).await;
}

async fn get_log(adb: &Adb<AdbRegexStrategy>, tx: mpsc::Sender<LineParts>) {
    loop {
        // Get next part and send 
        // let val = adb.next().await;
        // if let Some(&line_part) = val {
        //     tx.send(&ine_part).await;
        // }
    }
}

#[tokio::main]
async fn main() {
    let mut adb = Adb::new(AdbRegexStrategy::new());
    let (tx, mut rx) = mpsc::channel::<LineParts>(100);
    
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
