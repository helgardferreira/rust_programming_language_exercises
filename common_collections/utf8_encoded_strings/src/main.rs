fn main() {
    let mut s = String::new();
    s.push('h');
    println!("{:?}", s);

    // the to_string method
    // which is available on any type that implements the Display trait
    let data = "initial contents";

    let s = data.to_string();
    println!("{:?}", s);

    // String::from() is specifically for converting string literals
    let s = String::from("initial contents");
    println!("{:?}", s);

    // utf-8 encoded
    let utf8_array = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for str in utf8_array {
        println!("{:?}", str);
    }

    // appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('z');
    s.pop();
    println!("{:?}", s);

    // concatenation with the + operator or the format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // the + operator expects the value on the right to be a string slice
    // note s1 has been moved here and can no longer be used
    let s3 = s1 + &s2;
    println!("{:?}", s3);

    // the + operator uses the add method
    // whose signature looks something like this
    // (ignore enum and impl here -> it's for the compiler):
    enum Test {}

    impl Test {
        // note how self does not make use of &
        // it thus takes ownership over self
        // (which in this case is what's on the left side of the + operator
        fn add(self, s: &str) -> String {
            let some_string = String::new();
            // ...
            some_string
        }
    }

    /*
    But wait—the type of &s2 is &String, not &str, as specified in the
    second parameter to add. So why does the code compile?

    The reason we’re able to use &s2 in the call to add is that the compiler
    can coerce the &String argument into a &str.

    When we call the add method, Rust uses a deref coercion, which here
    turns &s2 into &s2[..].
    */

    // for more complicated string combining, we can use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}", s);

    // indexing into strings
    /*let s1 = String::from("hello");
    let h = s1[0];*/
    /*
    error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
      --> src/main.rs:88:13
       |
    88 |     let h = s1[0];
       |             ^^^^^ `std::string::String` cannot be indexed by `{integer}`
       |
       = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
    */

    // N.B. A String is a wrapper over a Vec<u8>
    let len = String::from("Hola").len();
    println!("{:?}", len); // -> 4 bytes long

    let len = String::from("Здравствуйте").len();
    println!("{:?}", len); // -> 24 bytes long, not 12!
    /*
    Rust’s answer is 24: that’s the number of bytes it takes to encode
    “Здравствуйте” in UTF-8, because each Unicode scalar value in that string
    takes 2 bytes of storage.
    */

    /*let hello = "Здравствуйте";
    let answer = &hello[0];*/
    /*
    error[E0277]: the type `str` cannot be indexed by `{integer}`
       --> src/main.rs:112:19
        |
    112 |     let answer = &hello[0];
        |                   ^^^^^^^^ string indices are ranges of `usize`
        |
        = help: the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`
        = note: you can use `.chars().nth()` or `.bytes().nth()`
                see chapter in The Book <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
        = note: required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`
    */

    // bytes and scalar values and ...
    // grapheme clusters (the closest thing to what we would call letters)

    // slicing strings
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{:?}", s); // -> Зд - this is due to UTF-8 encoding and byte sizes

    // slicing strings can be tricky!
    // let s = &hello[0..1];
    /*
    thread 'main' panicked at 'byte index 1 is not a char boundary;
    it is inside 'З' (bytes 0..2) of `Здравствуйте`',
    src/libcore/str/mod.rs:2069:5

    This is horrible! Not only did the compiler not catch this -
    but it also introduced a runtime error!

    Therefore you should use ranges to create string slices with caution!
    */

    // methods for iterating over strings
    // if you need to perform operations on individual Unicode scalar values,
    // the best way to do so is to use the chars method:

    for c in "नमस्ते".chars() {
        println!("{}", c);
    } // -> न म स ् त े - note the diacritics: ् and े !

    // the bytes method returns each raw byte,
    // which might be more appropriate for certain domains:
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    /*
    ->
    224
    164
    168
    224
    164
    174
    224
    164
    184
    224
    165
    141
    224
    164
    164
    224
    165
    135
    ...
    good God!
    */

    // strings are not so simple... :(
}
