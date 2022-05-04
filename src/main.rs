#![feature(generic_associated_types)]

mod logging;


use regex::Regex;
use std::io::Read;
use std::process::{Command, Stdio};

// format
// 05-01 22:45:25.653  3361  3382 E MesonHwc: HwcVsync vsync callback fail (0xa9a21590)-(-22)-(0xa9a37010)
// MM-DD HH:MM:SS PROCESS_ID PROCESS_ID LOG_LEVEL PREFIX: [MESSAGE]

#[tokio::main]
async fn main() {
    // let process = Command::new("adb")
    //     .args(["logcat"])
    //     .stdout(Stdio::piped())
    //     .spawn();
    // let child = process.unwrap();
    // let mut stdout = child.stdout.unwrap();

    // // Init buffer
    // let buffer: Vec<&str> = vec![" "; 1024];
    // let mut my_string = buffer.join("");

    // let max_count = 1;
    // let mut count = 0;

    // let re_pattern = r"(\d{2}-\d{2})\s.*(\d{2}:\d{2}:\d{2}.\d{3}).*";
    // let re = Regex::new(re_pattern).unwrap();

    // loop {

    //     if count >= max_count {
    //         break;
    //    }

    //     unsafe {
    //         let res = stdout.read(my_string.as_bytes_mut());
    //     }

    //     let parts: Vec<&str> = my_string.split("\n").collect();

    //     println!("{:?}", parts.len());
    //     for i in parts {
    //         println!("parts: {:?}", i);
    //         for capture in re.captures_iter(i) {
    //             re.captures
    //             println!("MM-DD: {:?}: ", &capture[0]);
    //             println!("HH: {:?}: ", &capture[0]);
    //         }
    //     }
        
        
    //     count += 1;
    // }

    // println!("{:?}", process.stdout);
    // println!("Hello, world!");
}
