/*
Boxes don't have performance overhead, other than storing their data on the heap
instead of on the stack. But they don't have many extra capabilities either.
You'll use them most often in these situations:

    - When you have a type whose size can't be known at compile time and you
      want to use a value of that type in a context that requires an exact size
    - When you have a large amount of data and you want to transfer ownership but
      ensure the data won't be copied when you do so
    - When you want to own a value and you care only that it's a type that
      implements a particular trait rather than being of a specific type
*/
use crate::List::{Cons, Nil};
use std::borrow::Borrow;

fn main() {
    // using a Box<T> to store data on the heap
    // N.B. the below example is not a good use case at all!
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil))
            ))
        ),
    );

    construct_function_consumer(&list);
} // b is de-allocated here (goes out of scope) - just like any other owned value

// enabling recursive types with boxes
/*
At compile time, Rust needs to know how much space a type takes up. One type
whose size can't be known at compile time is a recursive type, where a value can
have as part of itself another value of the same type. Because this nesting of
values could theoretically continue infinitely, Rust doesn't know how much space
a value of a recursive type needs. However, boxes have a known size, so by
inserting a box in a recursive type definition, you can have recursive types.
*/
// cons list type
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// by default (without Box<T>) this will not compile:
// illustration: https://doc.rust-lang.org/book/img/trpl15-01.svg
/*
error[E0072]: recursive type `List` has infinite size
  --> src\main.rs:30:1
   |
30 | enum List {
   | ^^^^^^^^^ recursive type has infinite size
31 |     Cons(i32, List),
   |               ---- recursive without indirection
   |
   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable
*/
// illustration with Box<T>: https://doc.rust-lang.org/book/img/trpl15-02.svg

fn construct_function_consumer(foo: &List) {
    match foo {
        Cons(val, inner_list) => {
            println!("{}", val);
            construct_function_consumer(inner_list.borrow());
        }
        Nil => {}
    }
}
