fn main() {
    /*
    The main aim of lifetimes is to prevent dangling references, which cause a
    program to reference data other than the data it's intended to reference.
    */
    /*
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
                          // ---------+
    */
    /*
    error[E0597]: `x` does not live long enough
      --> src\main.rs:10:13
       |
    10 |         r = &x;
       |             ^^ borrowed value does not live long enough
    11 |     }
       |     - `x` dropped here while still borrowed
    12 |
    13 |     println!("r: {}", r)
       |                       - borrow later used here
    */
    /*
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+
    */

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence
    };

    println!("{:?}", i);

    // the static lifetime
    /*
    One special lifetime we need to discuss is 'static, which means that this
    reference can live for the entire duration of the program.
    */
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

/*
Lifetime Annotation Syntax

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/
// if we don't specify a lifetime:
/*
error[E0106]: missing lifetime specifier
  --> src\main.rs:47:33
   |
47 | fn longest(x: &str, y: &str) -> &str {
   |               ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
47 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
*/

// lifetime annotations in function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
In practice, this means that the lifetime of the reference returned by the longest
function is the same as the smaller of the lifetimes of the references passed in.
*/

// lifetime annotations in struct definitions
/*
This annotation means an instance of ImportantExcerpt can’t outlive the reference
it holds in its part field.
*/
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// generic type parameters, trait bounds, and lifetimes together
use std::fmt::Display;

/*
This is the longest function from before that returns the longer of two string
slices. But now it has an extra parameter named ann of the generic type T, which
can be filled in by any type that implements the Display trait as specified by the
where clause. This extra parameter will be printed before the function compares
the lengths of the string slices, which is why the Display trait bound is
necessary. Because lifetimes are a type of generic, the declarations of the
lifetime parameter 'a and the generic type parameter T go in the same list inside
the angle brackets after the function name.
*/
fn longest_with_an_announcement<'a, T>
(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
