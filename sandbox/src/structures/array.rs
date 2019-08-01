fn main() {
    let array1 = [false, true, false];
    let array2 = [0.0, -1.0, 1.0, 0.5]; // [f64; 4] 型

    // lenで配列の長さを取得
    println!("{}", array1.len());

    // 0で初期化された長さ10 の配列を作成
    let array3 = [0; 10];
    // 配列とかをprintするときは{:?}を指定
    println!("{:?}", array3);

    let array4 = ['H', 'e', 'l', 'l', 'o'];
    println!("{}", array4[1]);
    let mut index = 0;
    // indexは定数じゃなくても(変数でも)大丈夫
    println!("{}", array4[index]);
    // 定数とかでindex指定して溢れているとコンパイルエラー
    //println!("{}", array4[5]);
    //index = 5; // 変数とかだじ実行時に実行時エラー(パニック)になる
    //println!("{}", array4[index]);

    assert_eq!(array4.get(0), Some(&'H')); // getだとoptoinで取れる (中身は要素の参照)
    assert_eq!(array4.get(5), None); // 範囲外だとNone

    let array5 = ['a'; 50]; //'a' 50こで初期化
    for ch in array5.iter() {
        print!("{}", ch);
    }
    println!("");
    // 可変イテレータも作れる
    let mut array6 = [1; 30];
    for n in array6.iter_mut() {
        *n += 2;
        print!("{}", n);
    }
    println!("");
}
