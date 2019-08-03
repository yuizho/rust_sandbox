fn main() {
    // strリテラル
    let s1 = "abc1"; // &'static str型
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let line_end = "途中で改行すると
    改行やスペースが入る";
    println!("{}", line_end);

    let no_line_end = "行末にバックスラッシュつけると\
                       改行などがはいらない";
    println!("{}", no_line_end);

    let emoji = "もちろん絵文字\u{1f600}も使える";
    println!("{}", emoji);

    // strスライス
    let str1 = "abcdefg";
    assert_eq!(str1.get(0..1), Some("a"));
    for ch in str1.chars() {
        println!("{}", ch);
    }

    // 可変なstr
    // 文字列リテラルをStringへ変換し、底から&mut strを取り出す
    let mut mut_str1 = "abcあいう".to_string(); // String型
    let mut_str2 = mut_str1.as_mut_str(); // &mut str型
    mut_str2.make_ascii_uppercase();
    println!("{}", mut_str2);
    println!("{}", mut_str1); // 元のStringも変更されている
}
