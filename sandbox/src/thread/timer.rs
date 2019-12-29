extern crate termion;

use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const TERMINATE: &str = "terminate";

fn main() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    sender.send(TERMINATE).unwrap();
                    break;
                }
                Key::Ctrl(c) => {
                    if c == 'c' {
                        sender.send(TERMINATE).unwrap();
                        break;
                    }
                }
                _ => (),
            }
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    for duration in (1..1500).rev() {
        if check_if_interrupted(&receiver) {
            // rawモードを解除
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::cursor::Show
            )
            .unwrap();
            return;
        }
        write!(
            stdout,
            "🍅🍅🍅{}{}{}",
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

fn check_if_interrupted(receiver: &Receiver<&str>) -> bool {
    match receiver.try_recv() {
        Ok(message) => message == TERMINATE,
        _ => false,
    }
}
