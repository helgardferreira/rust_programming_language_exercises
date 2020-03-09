/*
Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they
turn it on. Others can come into the room and watch the TV. When the last person
leaves the room, they turn off the TV because it's no longer being used. If someone
turns off the TV while others are still watching it, there would be uproar from
the remaining TV watchers!

NOTE! Rc<T> is only for use in single-threaded scenarios!
When we discuss concurrency in Chapter 16, we'll cover how to do reference counting
in multi-threaded programs.
*/

// using Rc<T> to share data
// illustration: https://doc.rust-lang.org/book/img/trpl15-03.svg
/*
We'll create list a that contains 5 and then 10. Then we'll make two more lists:
b that starts with 3 and c that starts with 4. Both b and c lists will then
continue on to the first a list containing 5 and 10. In other words, both lists
will share the first list containing 5 and 10.

Trying to implement this scenario using our definition of List with Box<T> won't
work, as shown below:
*/

// attempt using Box<T> smart pointer
/*enum List {
    Cons(i32, Box<List>),
    Nil,
}*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::borrow::Borrow;
use std::rc::Rc;

fn main() {
    /*let a = Cons(
        5, Box::new(Cons(
            10, Box::new(Nil),
        )),
    );

    let b = Cons(3, Box::new(a));*/
    /*
    The following will not work with Box<T> (due to attempting to have multiple
    smart pointers to the same value).
    */
    /*let c = Cons(4, Box::new(a));*/
    /*
    error[E0382]: use of moved value: `a`
      --> src\main.rs:45:30
       |
    34 |     let a = Cons(
       |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
    ...
    40 |     let b = Cons(3, Box::new(a));
       |                              - value moved here
    ...
    45 |     let c = Cons(4, Box::new(a));
       |                              ^ value used here after move
    */

    let a = Rc::new(Cons(
        5, Rc::new(Cons(
            10, Rc::new(Nil),
        )),
    ));

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    /*
    We could have called a.clone() rather than Rc::clone(&a), but Rust's
    convention is to use Rc::clone in this case. The implementation of Rc::clone
    doesn't make a deep copy of all the data like most types' implementations of
    clone do. The call to Rc::clone only increments the reference count, which
    doesn't take much time. Deep copies of data can take a lot of time. By using
    Rc::clone for reference counting, we can visually distinguish between the
    deep-copy kinds of clones and the kinds of clones that increase the reference
    count. When looking for performance problems in the code, we only need to
    consider the deep-copy clones and can disregard calls to Rc::clone.
    */

    construct_function_consumer(&b);
    construct_function_consumer(&c);

    // cloning a Rc<T> increases the reference count
    let a = Rc::new(Cons(
        5, Rc::new(Cons(
            10, Rc::new(Nil),
        )),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn construct_function_consumer(foo: &List) {
    match foo {
        Cons(val, inner_list) => {
            println!("{}", val);
            construct_function_consumer(inner_list.borrow());
        }
        Nil => {}
    }
}
