fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word1 = first_word(&my_string_literal[..]);

    println!("my_string_literal: {}", my_string_literal);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word2 = first_word(my_string_literal);

    println!("the first words are: {}, {}, {}", word, word1, word2);

    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];

    // println!("the slice is {}", *slice.to_string());
}
