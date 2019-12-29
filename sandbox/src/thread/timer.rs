extern crate termion;

use std::io::{stdout, Write};
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{clear, color};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    for duration in (1..1500).rev() {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(2, 1),
            clear::All,
            convert_to_min(duration)
        );
        stdout.flush().unwrap();

        // https://crates.io/crates/spin_sleep
        spin_sleep::sleep(Duration::from_secs(1));
    }
}

fn convert_to_min(duration: i32) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}
