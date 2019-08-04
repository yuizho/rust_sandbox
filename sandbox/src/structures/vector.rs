fn main() {
    let v1 = vec![false, true, false]; // Vec<bool>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5]; // Vec<f64>型
    assert_eq!(v2.len(), 4);

    // 0で100要素を初期化
    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);

    // mutableなベクタ
    let mut mut_vec1 = vec!['a', 'b', 'c'];
    mut_vec1.push('d');
    mut_vec1.push('e');
    println!("{:?}", mut_vec1);

    assert_eq!(mut_vec1.pop(), Some('e'));

    mut_vec1.insert(1, 'f');
    mut_vec1.remove(2);
    println!("{:?}", mut_vec1);

    // 別のベクタへの要素の移動
    let mut mut_vec2 = vec!['g', 'h'];
    mut_vec1.append(&mut mut_vec2);
    println!("mut_vec1: {:?}", mut_vec1);
    println!("mut_vec2{:?}", mut_vec2); // 全要素がmut_vec1へ移動して、空になる

    // 別のベクタへの要素のコピー
    let immutable_vec = vec!['i', 'j'];
    mut_vec1.extend_from_slice(&immutable_vec);
    println!("mut_vec1: {:?}", mut_vec1);
    println!("mut_vec2{:?}", immutable_vec); // immutable_vecは変更なし (各要素がコピーされる)
}
