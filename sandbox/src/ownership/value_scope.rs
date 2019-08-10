use std::ops::Drop;

fn main() {
    #[derive(Debug)]
    struct Parent(usize, Child, Child);
    impl Drop for Parent {
        fn drop(&mut self) {
            println!("Dropping {:?}", self);
        }
    }

    #[derive(Debug)]
    struct Child(usize);
    impl Drop for Child {
        fn drop(&mut self) {
            println!("Dropping {:?}", self);
        }
    }

    // 所有権の基礎
    let p1 = Parent(1, Child(11), Child(12)); // Child(11), Child(12)の所有者はp1
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2); // ココで破棄される
    }
    println!("(b) {:?}, ", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(a) p1: {:?}, p3: {:?}", p1, p3); // p1寄り先にココで破棄される

    // move semantics
    println!("----------------- move semantics");
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // p1は値の所有権を失っているのでココでやるとエラー

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1); // 別の値の所有権を持つのでココではエラーにならない

    // copy semantics
    println!("----------------- copy semantics");
    #[derive(Debug, Copy, Clone)]
    struct ParentCopy(usize, ChildCopy, ChildCopy);
    #[derive(Debug, Copy, Clone)]
    struct ChildCopy(usize);

    let pc1 = ParentCopy(1, ChildCopy(11), ChildCopy(12));
    let pc2 = pc1;
    println!("pc2: {:?}", pc2);
    println!("pc1: {:?}", pc1);
}
