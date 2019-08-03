fn main() {
    fn print_info(name: &str, sl: &[char]) {
        println!(
            " {:9} - {}, {:?}, {:?}, {:?}",
            name,
            sl.len(),   // usize
            sl.first(), // Option<char>
            sl[1],
            sl.last() // Option<char>
        );
    }
    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);

    print_info("&a1[..]", &a1[..]); // 全要素スライス
    print_info("&ai", &a1);
    print_info("&ai[1...3", &a1[1..3]); // 長さ2のスライス

    // 可変スライス
    let mut array1 = [5, 4, 3, 2]; // 配列 [i32; 4]型
    let slice1 = &mut array1[1..3]; // スライス &mut [i32]型
    slice1[0] = 6;
    slice1[1] *= 10;
    slice1.swap(0, 1);
    println!("{:?}", slice1);
    println!("{:?}", array1); // スライスを通じてものと配列の内容も書き換わる

    // 配列から可変スライスへ強制型変換
    let mut array2 = [1, 2, 3, 4, 5, 6];
    array2[2..6].reverse(); // &mut を省略しても強制的にスライスに型変換される
    println!("{:?}", array2);
}
