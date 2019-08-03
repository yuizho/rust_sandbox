fn main() {
    // リファレンス
    let mut n = 0;
    println!("main: n={}", n);

    fn f1(mut n: u32) {
        n = 1;
        println!("f1: n={}", n);
    }

    fn f2(n_ptr: &mut u32) {
        println!("f2: n_ptr={}", n_ptr);

        // *で参照先にアクセスできる。参照外しという。
        *n_ptr = 2;
        println!("f2: *n_ptr={}", *n_ptr);
    }

    f1(n);
    println!("main: n={}", n);

    // 可変ポインタを設定
    f2(&mut n);
    println!("main: n={}", n);

    let c1 = 'A';
    let c1_ptr = &c1; // イミュータブル参照
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    assert_eq!(*n1_ptr, 0);

    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);

    let mut c1 = 'A';
    let c1_ptr = &mut c1;
    assert_eq!(*c1_ptr, 'A'); //  参照外しによるポインタの指す値の取り出し
    *c1_ptr = 'B'; // 参照外しによるポインタの指す値の書き換え
    assert_eq!(c1, 'B');
}
