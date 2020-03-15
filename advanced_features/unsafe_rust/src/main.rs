/*
To switch to unsafe Rust, use the unsafe keyword and then start a new block that
holds the unsafe code. You can take four actions in unsafe Rust, called unsafe
superpowers, that you can't in safe Rust. Those superpowers include the ability to:

    - Dereference a raw pointer
    - Call an unsafe function or method
    - Access or modify a mutable static variable
    - Implement an unsafe trait
    - Access fields of unions

It's important to understand that unsafe doesn't turn off the borrow checker or
disable any other of Rust's safety checks: if you use a reference in unsafe code,
it will still be checked. The unsafe keyword only gives you access to these four
features that are then not checked by the compiler for memory safety. You'll still
get some degree of safety inside of an unsafe block.

In addition, unsafe does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: the intent is
that as the programmer, you'll ensure the code inside an unsafe block will access
memory in a valid way.
*/

// using extern to call external code from C
extern "C" {
    fn abs(input: i32) -> i32;
}

// global static variable
static HELLO_WORLD: &str = "Hello, world!";

// global and mutable static variable
static mut COUNTER: u32 = 0;

fn main() {
    /*
    Dereferencing a Raw Pointer

    Different from references and smart pointers, raw pointers:

        - Are allowed to ignore the borrowing rules by having both immutable and
          mutable pointers or multiple mutable pointers to the same location
        - Aren't guaranteed to point to valid memory
        - Are allowed to be null
        - Don't implement any automatic cleanup
    */

    let mut num = 5;

    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
    /*
    Notice that we don't include the unsafe keyword in this code. We can create
    raw pointers in safe code; we just can't dereference raw pointers outside an
    unsafe block, as you'll see in a bit.
    */

    /*
    Next, we'll create a raw pointer whose validity we can't be so certain of.
    The example below shows how to create a raw pointer to an arbitrary location
    in memory. Trying to use arbitrary memory is undefined: there might be data at
    that address or there might not, the compiler might optimize the code so there
    is no memory access, or the program might error with a segmentation fault.
    Usually, there is no good reason to write code like this, but it is possible.
    */
    let address = 0x012345usize;
    let _r = address as *const i32;

    /*
    Recall that we can create raw pointers in safe code, but we can't dereference
    raw pointers and read the data being pointed to. Below, we use the dereference
    operator * on a raw pointer that requires an unsafe block.
    */
    let mut num = 5;

    let r1: *const i32 = &num;
    let r2: *mut i32 = &mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    /*
    Note also that in the examples above, we created *const i32 and *mut i32 raw
    pointers that both pointed to the same memory location, where num is stored.
    If we instead tried to create an immutable and a mutable reference to num, the
    code would not have compiled because Rust's ownership rules don't allow a
    mutable reference at the same time as any immutable references. With raw
    pointers, we can create a mutable pointer and an immutable pointer to the same
    location and change data through the mutable pointer, potentially creating a
    data race. Be careful!
    */

    /*
    Calling an Unsafe Function or Method
    */

    unsafe {
        dangerous();
    }
    /*
    We get the following error if we try to invoke an unsafe function without the
    unsafe keyword:
    */
    /*
    error[E0133]: call to unsafe function is unsafe and requires unsafe function
                  or block
      --> src\main.rs:87:5
       |
    87 |     dangerous();
       |     ^^^^^^^^^^^ call to unsafe function
       |
       = note: consult the function's documentation for information on how to
               avoid undefined behavior
    */

    /*
    Creating a Safe Abstraction over Unsafe Code
    */
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using our own split_at_mut function
    let (a, b) = split_at_mut(&mut v, 4);

    println!("{:?}", a);
    println!("{:?}", b);

    /*
    Using extern Functions to Call External Code

    Sometimes, your Rust code might need to interact with code written in another
    language. For this, Rust has a keyword, extern, that facilitates the creation
    and use of a Foreign Function Interface (FFI). An FFI is a way for a
    programming language to define functions and enable a different (foreign)
    programming language to call those functions.

    The below example demonstrates how to set up an integration with the abs
    function from the C standard library. Functions declared within extern blocks
    are always unsafe to call from Rust code. The reason is that other languages
    don't enforce Rust's rules and guarantees, and Rust can't check them, so
    responsibility falls on the programmer to ensure safety.
    */
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    /*
    Accessing or Modifying a Mutable Static Variable
    */
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    // N.B. accessing and modifying mutable static variables is unsafe
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe function that makes use of unsafe code to modify global static variable
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}

// attempting a `safe` interpretation of the split_at_mut method
/*fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}*/
/*
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
   --> src\main.rs:127:11
    |
121 |   fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    |                          - let's call the lifetime of this reference `'1`
...
126 |       (&mut slice[..mid],
    |       -     ----- first mutable borrow occurs here
    |  _____|
    | |
127 | |      &mut slice[mid..])
    | |___________^^^^^_______- returning this value requires that `*slice` is
    |             |             borrowed for `'1`
    |             |
    |             second mutable borrow occurs here
*/
/*
Rust's borrow checker can't understand that we're borrowing different parts of
the slice; it only knows that we're borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two slices
aren't overlapping, but Rust isn't smart enough to know this. When we know code
is okay, but Rust doesn't, it's time to reach for unsafe code:
*/
// attempting an `unsafe` interpretation of the split_at_mut method
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(
             ptr.offset(mid as isize), len - mid)
        )
    }
}
/*
We use the len method to get the length of a slice and the as_mut_ptr method to
access the raw pointer of a slice. In this case, because we have a mutable slice
to i32 values, as_mut_ptr returns a raw pointer with the type *mut i32, which
we've stored in the variable ptr.

We keep the assertion that the mid index is within the slice. Then we get to the
unsafe code: the slice::from_raw_parts_mut function takes a raw pointer and a
length, and it creates a slice. We use this function to create a slice that starts
from ptr and is mid items long. Then we call the offset method on ptr with mid as
an argument to get a raw pointer that starts at mid, and we create a slice using
that pointer and the remaining number of items after mid as the length.

The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer
and must trust that this pointer is valid. The offset method on raw pointers is
also unsafe, because it must trust that the offset location is also a valid
pointer. Therefore, we had to put an unsafe block around our calls to
slice::from_raw_parts_mut and offset so we could call them. By looking at the code
and by adding the assertion that mid must be less than or equal to len, we can
tell that all the raw pointers used within the unsafe block will be valid pointers
to data within the slice. This is an acceptable and appropriate use of unsafe.

Note that we don't need to mark the resulting split_at_mut function as unsafe, and
we can call this function from safe Rust. We've created a safe abstraction to the
unsafe code with an implementation of the function that uses unsafe code in a safe
way, because it creates only valid pointers from the data this function has access
to.
*/

/*
Calling Rust Functions from Other Languages

Refer to the lib.rs file.
*/

/*
Implementing an Unsafe Trait
*/
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
/*
By using unsafe impl, we're promising that we'll uphold the invariants that the
compiler can't verify.

As an example, recall the Sync and Send marker traits we discussed in the
“Extensible Concurrency with the Sync and Send Traits” section in Chapter 16:
the compiler implements these traits automatically if our types are composed
entirely of Send and Sync types. If we implement a type that contains a type that
is not Send or Sync, such as raw pointers, and we want to mark that type as Send
or Sync, we must use unsafe. Rust can't verify that our type upholds the
guarantees that it can be safely sent across threads or accessed from multiple
threads; therefore, we need to do those checks manually and indicate as such with
unsafe.
*/

/*
When to Use Unsafe Code

Using unsafe to take one of the four actions (superpowers) just discussed isn't
wrong or even frowned upon. But it is trickier to get unsafe code correct because
the compiler can't help uphold memory safety. When you have a reason to use unsafe
code, you can do so, and having the explicit unsafe annotation makes it easier to
track down the source of problems if they occur.
*/
