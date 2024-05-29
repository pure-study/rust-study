fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    // Option 1:
    // let word = first_word(&my_string[0..6]);
    // Option 2:
    // let word = first_word(&my_string[..]);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    // Option 3:
    let word = first_word(&my_string);

    println!("the first word is: {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    // Option 1:
    // let word_of_literal = first_word(&my_string_literal[0..6]);
    // Option 2:
    let word_of_literal = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // Option 3:
    // let word_of_literal = first_word(my_string_literal);

    println!("the first word of literal is: {}", word_of_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}