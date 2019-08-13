use std::rc::Rc;

#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn main() {
    let mut rc1 = Rc::new(Child(1));
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2
        );
    } // スコープを抜けるとrc2は破棄される
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // 参照カウントが1(他に参照しているものがいない)ときは、可変の参照を取得できる
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1), // 参照カウントは1 (Weakはカウントされない)
        rc1,
        weak
    );

    // rc1をドロップする
    std::mem::drop(rc1);
    // 解法済みならNoneがかえる
    println!("(f) count: 0, weak.upgrade: {:?}", weak.upgrade());
}
