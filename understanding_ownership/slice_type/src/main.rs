fn main() {
    let mut s = String::from("hello world");

    let word = flawed_first_word(&s); // word will get the value 5
    println!("{:?}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // luckily, Rust has a solution to this problem: string slices!
    // a string slice is a reference to a part of a String:
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // illustration: https://doc.rust-lang.org/book/img/trpl04-06.svg

    println!("{}-{}", hello, world);

    // tip: if you want to start at the first index,
    // you can drop the value before the two periods:
    let s = String::from("hello");

    let slice1 = &s[..2]; // is the same as &s[0..2]
    // same for last byte of string:
    let slice2 = &s[3..s.len()]; // is the same as &s[3..s.len()]
    println!("{}", slice1);
    println!("{}", slice2);

    println!("{}", first_word(&String::from("hello world")));

    let my_string = String::from("hello world");

    // improved_first_word() works on slices of `String`s
    let word = improved_first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    // improved_first_word() works on slices of string literals
    let word = improved_first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = improved_first_word(my_string_literal);
    println!("{}", word);

    // other slices
    let a : [i32; 5] = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn flawed_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    };

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// improving the first_word function by using a string slice for the type of
// the s parameter
fn improved_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
