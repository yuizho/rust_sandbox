fn main() {
    let mut s1 = "ラズベリー".to_string();
    // String::fromでもOK
    let s2 = "ブラックベリー";

    s1.push_str("タルト");
    println!("{}", s1);
    s1.push('と');

    s1.push_str(s2);
    println!("{}", s1);

    // 数値 -> String
    let i = 42;
    assert_eq!(i.to_string(), "42"); // i32

    let f = 4.3 + 0.1; // f64
    println!("{}", f);
    assert_eq!(format!("{:.2}", f), "4.40");

    // &str -> 数値
    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse(); // 変換失敗するとErrけ結果が入る
    assert!(r2.is_err());
    println!("{:?}", r2);

    // char配列からString
    let cs = ['t', 'r', 'u', 's', 't']; // [char; 5]型
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(cs[1..].iter().collect::<String>(), "rust");
}
