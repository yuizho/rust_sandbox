use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    // thread localマクロはスレッドごとのstatic的な変数を導入する
    thread_local! {
        // 各スレッドでRABBITSにアクセスされたときに初期化サれる
        static RABBITS: RefCell<HashSet<& 'static str>>  = {
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    }

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
        println!("別スレッド: {:?}", rb.borrow());
    });

    // 別スレッドを起動してRABBITSへ要素つ追加する
    std::thread::spawn(|| RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")))
        .join()
        .expect("Thread error");

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト"));
        println!("メインスレッド: {:?}", rb.borrow());
    });
}
