fn main() {
    /*
     Ownership Rules
     - Each value in Rust has a variable that’s called its owner.
     - There can only be one owner at a time.
     - When the owner goes out of scope, the value will be dropped.
     */

    // variable scope
    // s is not valid here, it's not yet declared
    let s = "hello";   // s is valid from this point forward

    println!("{}", s);

    // the String type
    // this type is allocated on the heap and as such is able to
    // store an amount of text that is unknown to us at compile time
    // you can create a String from a string literal using the from function:
    // the :: operator allows us to namespace this 'from' function under the String type
    let s = String::from("hello");

    println!("{}", s);

    // this kind of string _can_ be mutated:
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);

    /*
     With the String type, in order to support a mutable, growable piece of text,
     we need to allocate an amount of memory on the heap, unknown at compile time,
     to hold the contents. This means:

     - The memory must be requested from the operating system at runtime.
     - We need a way of returning this memory to the operating system when
       we’re done with our String.

     That first part is done by us: when we call String::from, its implementation
     requests the memory it needs. This is pretty much universal in programming languages.

     However, the second part is different. In languages with a garbage collector (GC),
     the GC keeps track and cleans up memory that isn’t being used anymore, and we don’t
     need to think about it. Without a GC, it’s our responsibility to identify when
     memory is no longer being used and call code to explicitly return it,
     just as we did to request it. Doing this correctly has historically been a
     difficult programming problem. If we forget, we’ll waste memory. If we do it
     too early, we’ll have an invalid variable. If we do it twice, that’s a bug too.
     We need to pair exactly one allocate with exactly one free.

     Rust takes a different path: the memory is automatically returned once the
     variable that owns it goes out of scope.
     */

    // Ways Variables and Data Interact: Move
    // illustrations:
    // https://doc.rust-lang.org/book/img/trpl04-01.svg
    // https://doc.rust-lang.org/book/img/trpl04-02.svg

    /*
     This is a problem: when s2 and s1 go out of scope, they will both try to free
     the same memory. This is known as a double free error and is one of the memory
     safety bugs we mentioned previously. Freeing memory twice can lead to
     memory corruption, which can potentially lead to security vulnerabilities.

     Instead of trying to copy the allocated memory, Rust considers s1 to no longer
     be valid and, therefore, Rust doesn't need to free anything when s1
     goes out of scope. Check out what happens when you try to use s1 after s2 is
     created; it won’t work:
     */
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    */
    /*
    error[E0382]: borrow of moved value: `s1`
      --> src/main.rs:76:28
       |
    73 |     let s1 = String::from("hello");
       |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    74 |     let s2 = s1;
       |              -- value moved here
    75 |
    76 |     println!("{}, world!", s1);
       |                            ^^ value borrowed here after move
    */
    // s1 was moved into s2:
    // https://doc.rust-lang.org/book/img/trpl04-04.svg

    // Ways Variables and Data Interact: Clone
    /*
     If we do want to deeply copy the heap data of the String,
     not just the stack data, we can use a common method called clone.
     */
    // here's an example of the clone method in action:
    // illustration: https://doc.rust-lang.org/book/img/trpl04-03.svg
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    /*
     So what types are Copy? You can check the documentation for the given type
     to be sure, but as a general rule, any group of simple scalar values can be
     Copy, and nothing that requires allocation or is some form of resource is
     Copy. Here are some of the types that are Copy:

     - All the integer types, such as u32.
     - The Boolean type, bool, with values true and false.
     - All the floating point types, such as f64.
     - The character type, char.
     - Tuples, if they only contain types that are also Copy. For example,
       (i32, i32) is Copy, but (i32, String) is not.
     */

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);          // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                         // x comes into scope

    makes_copy(x);              // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    /*
     Taking ownership and then returning ownership with every function is a bit
     tedious. What if we want to let a function use a value but not take ownership?
     It’s quite annoying that anything we pass in also needs to be passed back if
     we want to use it again, in addition to any data resulting from the body of
     the function that we might want to return as well.
     */
    // It’s possible to return multiple values using a tuple:

    let z1 = String::from("hello");

    let (s2, len) = calculate_lengths(z1);

    println!("The length of '{}' is {}.", s2, len)
    // but it is preferable to make use of references
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_lengths(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
