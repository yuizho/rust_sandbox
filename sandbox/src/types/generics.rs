fn largest_by_borrow<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_by_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // with borrow
    let number_list = vec![34, 50, 25, 100, 65, 101];
    let result = largest_by_borrow(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = largest_by_borrow(&char_list);
    println!("The largest char is {}", result);

    println!("-----------------------------");

    // with copy
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_by_copy(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_by_copy(&char_list);
    println!("The largest char is {}", result);
}
