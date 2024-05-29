fn main() {
    // test_invalid_after_move();
    println!("=============");
    test_clone();
    println!("=============");
    test_stack_only_data_copy();
}

/* fn test_invalid_after_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 = {}", s1);   // s1 is invalid here after move
} */

fn test_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn test_stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);  // x is still valid
}
