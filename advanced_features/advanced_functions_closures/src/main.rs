/*
Function Pointers

We've talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a function
you've already defined rather than defining a new closure. Doing this with function
pointers will allow you to use functions as arguments to other functions. Functions
coerce to the type fn (with a lowercase f), not to be confused with the Fn closure
trait. The fn type is called a function pointer.
*/
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // inline closure example
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("{:?}", list_of_strings);

    // compared to using a function instead of a closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_strings);

    /*
    Note that we must use the fully qualified syntax that we talked about earlier
    in the "Advanced Traits" section because there are multiple functions
    available named to_string. Here, we're using the to_string function defined in
    the ToString trait, which the standard library has implemented for any type
    that implements Display.

    We have another useful pattern that exploits an implementation detail of tuple
    structs and tuple-struct enum variants. These types use () as initializer
    syntax, which looks like a function call. The initializers are actually
    implemented as functions returning an instance that's constructed from their
    arguments. We can use these initializer functions as function pointers that
    implement the closure traits, which means we can specify the initializer
    functions as arguments for methods that take closures, like so:
    */
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)
            .collect();

    println!("{:?}", list_of_statuses);

    println!("{:?}", returns_closure(10)(5));
}

/*
Returning Closures

Closures are represented by traits, which means you can't return closures directly.
In most cases where you might want to return a trait, you can instead use the
concrete type that implements the trait as the return value of the function. But
you can't do that with closures because they don't have a concrete type that is
returnable; you're not allowed to use the function pointer fn as a return type,
for example.

The following code tries to return a closure directly, but it won't compile:
*/
/*fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}*/
/*
error[E0277]: the size for values of type `(dyn std::ops::Fn(i32) -> i32 + 'static)`
cannot be known at compilation time
  --> src\main.rs:81:25
   |
81 | fn returns_closure() -> Fn(i32) -> i32 {
   |                         ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for
           `(dyn std::ops::Fn(i32) -> i32 + 'static)`
   = note: to learn more, visit
           <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
*/
/*
The error references the Sized trait again! Rust doesn't know how much space it
will need to store the closure. We saw a solution to this problem earlier. We can
use a trait object:
*/
fn returns_closure(y: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + y)
}
