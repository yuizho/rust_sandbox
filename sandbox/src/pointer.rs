fn main() {
    // 関数ポインタ
    fn double(n: i32) -> i32 {
        n + n
    }
    fn abs(n: i32) -> i32 {
        if n >= 0 {
            n
        } else {
            -n
        }
    }
    // 型注釈が無いと関数定義型だと推論されて、
    // abs代入時にコンパイルエラー
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);
    f = abs;
    assert_eq!(f(-42), 42);
}
