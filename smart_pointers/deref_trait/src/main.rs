/*
Note: there's one big difference between the MyBox<T> type we're about to build
and the real Box<T>: our version will not store its data on the heap. We are
focusing this example on Deref, so where the data is actually stored is less
important than the pointer-like behavior.
*/
// defining our own smart pointer
// tuple struct hybrid - has a name but their fields don't (basically named tuples)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// treating a type like a reference by implementing the Deref trait
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // associated type for the Deref trait to use
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
    /*
    The reason the deref method returns a reference to a value, and that the plain
    dereference outside the parentheses in *(y.deref()) is still necessary, is
    the ownership system. If the deref method returned the value directly instead
    of a reference to the value, the value would be moved out of self. We don't
    want to take ownership of the inner value inside MyBox<T> in this case or in
    most cases where we use the dereference operator.
    */
}

fn main() {
    /*
    Deref coercion makes it possible to call hello with a reference to a value of
    type MyBox<String>.

    Rust can turn &MyBox<String> into &String by calling deref.
    */
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    /*
    If Rust didn't implement deref coercion, we would have to write the following
    code:
    */
    let verbose_str = &(*m)[..];
    hello(verbose_str);
}

// implicit deref coercions with functions
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn deference_int() {
        // following the pointer to the value with the dereference operator
        let x = 5;
        let y = MyBox::new(x); // *(y.deref())

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
