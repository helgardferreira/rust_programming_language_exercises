/*
Using the Newtype Pattern for Type Safety and Abstraction

The newtype pattern is useful for tasks beyond those we've discussed so far,
including statically enforcing that values are never confused and indicating the
units of a value. An example would be the Millimeters and Meters structs wrapping
u32 values in a newtype. If we wrote a function with a parameter of type
Millimeters, we couldn't compile a program that accidentally tried to call that
function with a value of type Meters or a plain u32.

Another use of the newtype pattern is in abstracting away some implementation
details of a type: the new type can expose a public API that is different from the
API of the private inner type if we used the new type directly to restrict the
available functionality, for example.

Newtypes can also hide internal implementation. For example, we could provide a
People type to wrap a HashMap<i32, String> that stores a person's ID associated
with their name. Code using People would only interact with the public API we
provide, such as a method to add a name string to the People collection; that code
wouldn't need to know that we assign an i32 ID to names internally. The newtype
pattern is a lightweight way to achieve encapsulation to hide implementation
details, which we discussed in the "Encapsulation that Hides Implementation
Details" section of Chapter 17.
*/

/*
Creating Type Synonyms with Type Aliases
*/
type Kilometers = i32;

/*
Type aliases are also commonly used with the Result<T, E> type for reducing
repetition. Consider the std::io module in the standard library. I/O operations
often return a Result<T, E> to handle situations when operations fail to work. This
library has a std::io::Error struct that represents all possible I/O errors. Many
of the functions in std::io will be returning Result<T, E> where the E is
std::io::Error, such as these functions in the Write trait:
*/
// use std::io::Error;
use std::io::Result;
use std::fmt;

/*pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}*/
/*
The Result<..., Error> is repeated a lot. As such, std::io has this type of alias
declaration:

type Result<T> = std::result::Result<T, std::io::Error>;
*/
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
/*
The type alias helps in two ways: it makes code easier to write and it gives us a
consistent interface across all of std::io. Because it's an alias, it's just
another Result<T, E>, which means we can use any methods that work on
Result<T, E> with it, as well as special syntax like the ? operator.
*/

fn main() {
    /*
    Now, the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters
    types we created in earlier, Kilometers is not a separate, new type. Values that
    have the type Kilometers will be treated the same as values of type i32:
    */

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    /*
    The main use case for type synonyms is to reduce repetition. For example, we
    might have a lengthy type like this:

    Box<dyn Fn() + Send + 'static>
    */

    let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // a type alias makes this code more manageable by reducing the repetition
    let f: Thunk = Box::new(|| println!("hi"));
    (*f)();

    /*
    The Never Type that Never Returns
    */
    /*fn bar() -> ! {
        // --snip--
    }*/
    /*
    This code is read as "the function bar returns never." Functions that return never
    are called diverging functions. We can't create values of the type ! so bar can
    never possibly return.

    But what use is a type you can never create values for?
    */
    let guess = "34";

    let _guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("invalid input!"),
    };
    /*
    The continue keyword and the panic! macro has a ! return value. That is, when
    Rust computes the type of guess, it looks at both match arms, the former with
    a value of u32 and the latter with a ! value. Because ! can never have a value,
    Rust decides that the type of guess is u32.
    */

    /*
    Dynamically Sized Types and the Sized Trait

    Due to Rust's need to know certain details, such as how much space to allocate
    for a value of a particular type, there is a corner of its type system that
    can be confusing: the concept of dynamically sized types. Sometimes referred
    to as DSTs or unsized types, these types let us write code using values whose
    size we can know only at runtime.

    Let's dig into the details of a dynamically sized type called str, which
    we've been using throughout the book. That's right, not &str, but str on its
    own, is a DST. We can't know how long the string is until runtime, meaning we
    can't create a variable of type str, nor can we take an argument of type str.
    Consider the following code, which does not work:
    */
    /*let s1: str = "Hello there!";
    let s2: str = "How's it going?";*/
    /*
    --snip--
        error[E0277]: the size for values of type `str` cannot be known at
                      compilation time
       --> src\main.rs:137:9
        |
    137 |     let s2: str = "How's it going?";
        |         ^^ doesn't have a size known at compile-time
        |
        = help: the trait `std::marker::Sized` is not implemented for `str`
        = note: to learn more, visit
                <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
        = note: all local variables must have a statically known size
        = help: unsized locals are gated as an unstable feature
    */
    /*
    So what do we do? In this case, you already know the answer: we make the
    types of s1 and s2 a &str rather than a str. Recall that in the
    "String Slices" section of Chapter 4, we said the slice data structure stores
    the starting position and the length of the slice.

    So although a &T is a single value that stores the memory address of where the
    T is located, a &str is two values: the address of the str and its length. As
    such, we can know the size of a &str value at compile time: it's twice the
    length of a usize. That is, we always know the size of a &str, no matter how
    long the string it refers to is. In general, this is the way in which
    dynamically sized types are used in Rust: they have an extra bit of metadata
    that stores the size of the dynamic information. The golden rule of
    dynamically sized types is that we must always put values of dynamically
    sized types behind a pointer of some kind.
    */
}

fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
    Box::new(|| println!("Hello World"))
}

// type alias for long type
/*
Thunk is a word for code to be evaluated at a later time, so it's an appropriate
name for a closure that gets stored.
*/
type Thunk = Box<dyn Fn() + Send + 'static>;

fn _takes_thunk(_f: Thunk) {
    // --snip--
}

fn _returns_thunk() -> Thunk {
    // --snip--
    Box::new(|| println!("Hello World"))
}

// ?Sized trait bound
/*
A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read
this as “T may or may not be Sized.” This syntax is only available for Sized,
not any other traits.

Because the type might not be Sized, we need to use it behind some kind of
pointer. In this case, we’ve chosen to make the parameter a reference to T.
*/
fn _generic<T: ?Sized>(_t: &T) {
    // --snip--
}
