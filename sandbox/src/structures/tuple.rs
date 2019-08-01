fn main() {
    // 要素を指定しての値の取り出し
    let tuple1 = (88, true);
    println!("{} {}", tuple1.0, tuple1.1);

    // 要素の書き換え (可変)
    let mut mutable_tuple = (88, true);
    mutable_tuple.0 += 100;
    println!("{} {}", mutable_tuple.0, mutable_tuple.1);

    // patter matchで取り出し
    let (n1, b1) = (88, true);
    println!("{} {}", n1, b1);
}
