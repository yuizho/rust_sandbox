use std::time::Duration;

fn main() {
    for duration in (1..1500).rev() {
        println!("{}", convert_to_min(duration));
        spin_sleep::sleep(Duration::from_secs(1));
    }
}

fn convert_to_min(duration: i32) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}
