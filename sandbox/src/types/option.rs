fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a')); // あればSome(&値)
    assert_eq!(a1.get(4), None); // 範囲外はNone

    let mut o1 = Some(10);
    match o1 {
        Some(s) => println!("{}", s),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        println!("{}", s);
    }

    // コンビネータ
    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250)); // 値があればmapを適用
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None); // Noneのときは何もしない

    fn add_elems(s: &[i32]) -> Option<i32> {
        // 複数のOptionが全て値があるか調べつつ処理したいときは ? が便利。
        // Someなら値を取り出し、Noneなら関数から抜ける
        let s0 = s.get(0)?;
        let s3 = s.get(3)?;
        Some(s0 + s3)
    }

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[3, 11]), None);
}
