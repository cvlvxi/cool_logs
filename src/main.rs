#![feature(generic_associated_types)]

mod logging;


use crate::logging::adb::strategies::AdbRegexStrategy;
use crate::logging::adb::Adb;
use crate::logging::LineParts;
use crate::logging::Parser;

use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio::io::stdin;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};

async fn receive_logs(rx: &mut mpsc::Receiver<LineParts>) {
    while let Some(i) = rx.recv().await {
        println!("Got the line parts: {:?}", i);
    }
}

async fn io_handler(tx: mpsc::Sender<bool>) {
    let mut buffer = String::new();
    let mut stdin = BufReader::new(stdin());
    loop {
        stdin.read_line(&mut buffer).await;
        if buffer.contains("dog") {
            println!("here!");
            tx.send(true).await;
        }
        println!("{:?}", buffer);
    }
}

async fn get_logs(adb: &mut Adb<AdbRegexStrategy>, tx: mpsc::Sender<LineParts>, rx2: &mut mpsc::Receiver<bool>) {
    println!("Get logs");
    loop {
        let res = rx2.try_recv();
        if let Ok(val) = res {
            println!("Exiting log!!!!");
            break;
        } else {
        }

        // Get next part and send
        let val = adb.next().await;
        if let Some(line_part) = val {
            tx.send(line_part).await.unwrap();
        }

    }
    
    println!("I have quit!");
}

#[tokio::main]
async fn main() {
    let mut adb = Adb::new(AdbRegexStrategy::new());
    let (tx, mut rx) = mpsc::channel::<LineParts>(100);
    let (tx2, mut rx2) = mpsc::channel::<bool>(2);

    let a = tokio::spawn(async move {
        get_logs(&mut adb, tx, &mut rx2).await;
    });

    let b = tokio::spawn(async move {
        loop {
            receive_logs(&mut rx).await;
        }
    });

    let c = tokio::spawn(async move {
        io_handler(tx2).await;
    });

    // a.await;
    c.await;
}
