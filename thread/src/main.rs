use std::{thread, time};

fn main() {
    let n1 = 1200;
    let n2 = 1000;

    // spawnで別スレッドを作って、処理を実行
    let child = thread::spawn(move || heavy_calc("child", n2));

    let s1 = heavy_calc("main", n1);

    // 別スレッドのハンドルに対して、joinで処理の終了を待ち受ける
    match child.join() {
        Ok(s2) => println!("{} {}", s1, s2),
        Err(e) => println!("error: {:?}", e),
    }
}

fn heavy_calc(name: &str, n: u64) -> u64 {
    println!("{}: started.", name);
    thread::sleep(time::Duration::from_millis(n));
    let sum = (1..(n + 1)).sum();
    println!("{}: ended.", name);
    sum
}
