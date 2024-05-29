
fn main() {
    test_01();
    println!("====================");
    test_02_different_lifetimes();
}

fn test_02_different_lifetimes() {
    let string1 = String::from("long string is long");

    // let result: &str;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // won't work the next line:
    // println!("The longest string is {}", result);
}

fn test_01() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
