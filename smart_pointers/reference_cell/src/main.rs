/*
Interior mutability is a design pattern in Rust that allows you to mutate data even
when there are immutable references to that data; normally, this action is
disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code
inside a data structure to bend Rust's usual rules that govern mutation and
borrowing. We haven't yet covered unsafe code; we will in Chapter 19. We can use
types that use the interior mutability pattern when we can ensure that the
borrowing rules will be followed at runtime, even though the compiler can't
guarantee that. The unsafe code involved is then wrapped in a safe API, and the
outer type is still immutable.

The RefCell<T> type is useful when you're sure your code follows the borrowing
rules but the compiler is unable to understand and guarantee that.

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will
give you a compile-time error if you try using it in a multi-threaded context.
We'll talk about how to get the functionality of RefCell<T> in a multi-threaded
program in Chapter 16.

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

    - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
      single owners.
    - Box<T> allows immutable or mutable borrows checked at compile time;
      Rc<T> allows only immutable borrows checked at compile time;
      RefCell<T> allows immutable or mutable borrows checked at runtime.
    - Because RefCell<T> allows mutable borrows checked at runtime, you can
      mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the interior mutability pattern.
*/

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::borrow::Borrow;
use std::cell::RefCell;

fn main() {
    /*let x = 5;
    let y = &mut x;*/
    /*
    error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
      --> src\main.rs:34:13
       |
    33 |     let x = 5;
       |         - help: consider changing this to be mutable: `mut x`
    34 |     let y = &mut x;
       |             ^^^^^^ cannot borrow as mutable
    */

    /*
    N.B. remember to checkout the lib.rs file for a large amount of information on
    RefCell<T>
    */

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(
        Cons(
            Rc::clone(&value),
            Rc::new(Nil),
        )
    );

    let b = Cons(
        Rc::new(RefCell::new(6)),
        Rc::clone(&a),
    );

    let c = Cons(
        Rc::new(RefCell::new(10)),
        Rc::clone(&a),
    );

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    construct_function_consumer(&b);
    construct_function_consumer(&c);
}

// having multiple owners of mutable data by combining Rc<T> and RefCell<T>
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn construct_function_consumer(foo: &List) {
    match foo {
        Cons(val, inner_list) => {
            let val = (val.borrow() as &RefCell<i32>)
                .borrow();

            println!("{}", val);
            construct_function_consumer(inner_list.borrow());
        }
        Nil => {}
    };
}
