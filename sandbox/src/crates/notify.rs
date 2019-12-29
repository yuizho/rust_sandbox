use notify_rust::Notification;

fn main() {
    // https://crates.io/crates/notify-rust
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .show();
}
