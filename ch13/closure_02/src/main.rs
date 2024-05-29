use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    test_01();
    println!("===============");
    test_02();
    println!("===============");
    test_03();
    println!("\n===============");
    test_04();
    println!("===============");
    test_05();
}

fn test_01() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn test_02() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);  // not allowed
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn test_03() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn test_04() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

// fn test_not_working() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("by key called");

//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{:#?}", list);
// }

fn test_05() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
