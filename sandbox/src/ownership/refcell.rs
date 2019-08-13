use std::cell::RefCell;

#[derive(Debug)]
struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    println!("{:?}", b);
    rb.s.borrow_mut().push('a');
    assert!(b.s.try_borrow_mut().is_ok());
    println!("{:?}", b);
}
