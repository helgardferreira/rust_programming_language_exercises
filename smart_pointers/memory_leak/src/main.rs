/*
Rust's memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a memory leak).
Preventing memory leaks entirely is not one of Rust's guarantees in the same way
that disallowing data races at compile time is, meaning memory leaks are memory
safe in Rust. We can see that Rust allows memory leaks by using Rc<T> and
RefCell<T>: it's possible to create references where items refer to each other in
a cycle. This creates memory leaks because the reference count of each item in the
cycle will never reach 0, and the values will never be dropped.
*/

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

// list type that is capable of reference cycle
// (since it is a self-referential type)
#[derive(Debug)]
enum List {
    /*
    The second element in the Cons variant is RefCell<Rc<List>>, meaning that
    we want to modify which List value a Cons variant is pointing to.
    */
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // tail method to grab second entry in List (if the enum variant is Cons)
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(
        Cons(
            5,
            RefCell::new(Rc::new(Nil)),
        )
    );

    println!("a's initial rc count = {}", Rc::strong_count(&a));
    println!("a's next item = {:?}", a.tail());

    if let Some(link) = a.tail() {
        println!("a's next next item = {:?}", link.borrow().tail());
    }

    let b = Rc::new(
        Cons(
            10,
            RefCell::new(Rc::clone(&a)),
        )
    );

    println!("a's rc count after b's creation = {}", Rc::strong_count(&a));
    println!("b's initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b's rc count after changing a = {}", Rc::strong_count(&b));
    println!("a's rc count after changing a = {}", Rc::strong_count(&a));

    /*
    The reference count of the Rc<List> instances in both a and b are 2 after we
    change the list in a to point to b. At the end of main, Rust will try to drop
    b first, which will decrease the count of the Rc<List> instance in b by 1.

    However, because a is still referencing the Rc<List> that was in b, that
    Rc<List> has a count of 1 rather than 0, so the memory the Rc<List> has on
    the heap won’t be dropped. The memory will just sit there with a count of 1,
    forever.
    */

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
