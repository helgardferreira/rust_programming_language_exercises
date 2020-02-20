fn main() {
    let s1 = String::from("hello");

    // the ampersand (&) denotes a reference to a variable on the heap
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // immutable_change(&s1);

    let mut s2 = String::from("hello");

    mutable_change(&mut s2);
    println!("{:?}", s2);

    // mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data
    // in a particular scope
    /*
    let mut s3 = String::from("hello");

    let r1 = &mut s3;
    let r2 = &mut s3;

    println!("{}, {}", r1, r2);
    */
    /*
    error[E0499]: cannot borrow `s3` as mutable more than once at a time
      --> src/main.rs:22:14
       |
    21 |     let r1 = &mut s3;
       |              ------- first mutable borrow occurs here
    22 |     let r2 = &mut s3;
       |              ^^^^^^^ second mutable borrow occurs here
    23 |
    24 |     println!("{}, {}", r1, r2);
       |                        -- first borrow later used here
    */
    // This restriction allows for mutation but in a very controlled fashion
    // the benefit of having this restriction is that Rust can prevent data races
    // ... at compile time.
    /*
     A data race is similar to a race condition and happens
       when these three behaviours occur:
     - Two or more pointers access the same data at the same time.
     - At least one of the pointers is being used to write to the data.
     - There's no mechanism being used to synchronize access to the data.
    */

    // we can use curly brackets to create a new scope
    // allowing for multiple mutable references, just not simultaneous ones
    let mut s4 = String::from("hello");

    {
        let r1 = &mut s4;
        println!("{:?}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s4;
    println!("{:?}", r2);

    // a similar rule exists for combining mutable and immutable references
    /*
    let mut s5 = String::from("hello");

    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    let r3 = &mut s5; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */
    /*
    error[E0502]: cannot borrow `s5` as mutable because it is also borrowed as immutable
      --> src/main.rs:67:14
       |
    65 |     let r1 = &s5; // no problem
       |              --- immutable borrow occurs here
    66 |     let r2 = &s5; // no problem
    67 |     let r3 = &mut s5; // BIG PROBLEM
       |              ^^^^^^^ mutable borrow occurs here
    68 |
    69 |     println!("{}, {}, and {}", r1, r2, r3);
       |                                -- immutable borrow later used here
    */

    /*
    Note that a reference’s scope starts from where it is introduced and
    continues through the last time that reference is used. For instance,
    this code will compile because the last usage of the immutable references
    occurs before the mutable reference is introduced:
    */
    let mut s6 = String::from("hello");

    let r1 = &s6; // no problem
    let r2 = &s6; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s6; // no problem
    println!("{}", r3);

    // dangling references
    // let reference_to_nothing = dangle();
    /*
    error[E0106]: missing lifetime specifier
       --> src/main.rs:139:16
        |
    139 | fn dangle() -> &String {
        |                ^ help: consider giving it a 'static lifetime: `&'static`
        |
        = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    */

    println!("{:?}", no_dangle());
}

// here is how you would define and use a calculate_length function
// that has a reference to an object as a parameter
// instead of taking ownership of the value:
// illustration: https://doc.rust-lang.org/book/img/trpl04-05.svg
fn calculate_length(s: &String) -> usize { // s is a reference to a String, this is referred to as borrowing
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
//// it refers to, nothing special happens.

// immutable references (default)
// So what happens if we try to modify something we're borrowing?
/*
fn immutable_change(some_string: &String) {
    some_string.push_str(", world");
}
*/
// we get the following error
/*
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
  --> src/main.rs:23:5
   |
22 | fn immutable_change(some_string: &String) {
   |                        ------- help: consider changing this to be a mutable reference: `&mut std::string::String`
23 |     some_string.push_str(", world");
   |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
*/

// mutable references
fn mutable_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling references
/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
*/
//// Danger!

// the solution here is to return the String directly
// this works without any problems
// ownership is moved out, and nothing is de-allocated.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
