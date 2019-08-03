fn main() {
    let t1 = (3, "birds".to_string()); // (i32, String)方のタプル。スタックに置かれる
    let mut b1 = Box::new(t1); // Boxポインタを作る。タプルがヒープに移動する
    (*b1).0 += 1; // b1の参照はずして、実態のタプルの値を書き換え
    assert_eq!(*b1, (4, "birds".to_string()));
    println!("{:?}", &b1);
    // Box:newの後にt1へアクセスしようとするとエラーになる
    // println!("{:?}", &t1);
}
