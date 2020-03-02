use std::io::{ErrorKind, Read};
use std::io;
use std::fs;
use std::fs::File;
use std::error::Error;

/*
The main function is special, and there are restrictions on what its return type
must be. One valid return type for main is () (the default return type),
and conveniently, another valid return type is Result<T, E>, as shown here:
*/
fn main() -> Result<(), Box<dyn Error>> {
    // unrecoverable errors
    // manually calling the panic! macro
    /*panic!("crash and burn!");*/

    // panic macro being invoked from a library
    /*let v = vec![1, 2, 3];

    v[99];*/
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',
    /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
    */

    // with RUST_BACKTRACE=1
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',
    /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
    stack backtrace:
       0: backtrace::backtrace::libunwind::trace
                 at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
       1: backtrace::backtrace::trace_unsynchronized
                 at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
       2: std::sys_common::backtrace::_print_fmt
                 at src/libstd/sys_common/backtrace.rs:84
       3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
                 at src/libstd/sys_common/backtrace.rs:61
       4: core::fmt::ArgumentV1::show_usize
       5: std::io::Write::write_fmt
                 at src/libstd/io/mod.rs:1426
       6: std::sys_common::backtrace::_print
                 at src/libstd/sys_common/backtrace.rs:65
       7: std::sys_common::backtrace::print
                 at src/libstd/sys_common/backtrace.rs:50
       8: std::panicking::default_hook::{{closure}}
                 at src/libstd/panicking.rs:193
       9: std::panicking::default_hook
                 at src/libstd/panicking.rs:210
      10: std::panicking::rust_panic_with_hook
                 at src/libstd/panicking.rs:471
      11: rust_begin_unwind
                 at src/libstd/panicking.rs:375
      12: core::panicking::panic_fmt
                 at src/libcore/panicking.rs:84
      13: core::panicking::panic_bounds_check
                 at src/libcore/panicking.rs:62
      14: <usize as core::slice::SliceIndex<[T]>>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
      15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2657
      16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/liballoc/vec.rs:1871
      17: panic::main
                 at src/main.rs:8
      18: std::rt::lang_start::{{closure}}
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
      19: std::rt::lang_start_internal::{{closure}}
                 at src/libstd/rt.rs:52
      20: std::panicking::try::do_call
                 at src/libstd/panicking.rs:292
      21: __rust_maybe_catch_panic
                 at src/libpanic_unwind/lib.rs:78
      22: std::panicking::try
                 at src/libstd/panicking.rs:270
      23: std::panic::catch_unwind
                 at src/libstd/panic.rs:394
      24: std::rt::lang_start_internal
                 at src/libstd/rt.rs:51
      25: std::rt::lang_start
                 at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
      26: panic::main
    */

    // recoverable errors with Result
    /*enum Result<T, E> {
        Ok(T),
        Err(E),
    }*/

    /*let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };*/
    /*
    thread 'main' panicked at 'Problem opening the file:
    Os { code: 2, kind: NotFound, message: "The system cannot find the file
    specified." }', src\main.rs:88:13
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    error: process didn't exit successfully: `target\debug\panic.exe` (exit code: 101)
    */

    // adding an inner match expression to further evaluate the kind of error:
    /*let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };*/

    // lots of use of the match expression!
    // later we'll learn about closures - allowing our code to be more concise:
    /*let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });*/

    // shortcuts for panic on error: unwrap and expect
    // if the Result is the Err variant, unwrap will call the panic! macro for us
    // here is an example of unwrap in action:
    /*let f = File::open("hello.txt").unwrap();*/

    // another method, expect, which is similar to unwrap, lets us also choose
    // the panic! error message:
    /*let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");*/
    /*
    thread 'main' panicked at 'Failed to open hello.txt:
    Os { code: 2, kind: NotFound, message: "The sy
    stem cannot find the file specified." }', src\libcore\result.rs:1188:5
    */

    // let f = File::open("hello.txt")?;
    Ok(())
}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(num_bytes) => {
            println!("{:?}", num_bytes);
            Ok(s)
        }
        Err(e) => Err(e),
    }
}

// a shortcut for propagating errors: the ? operator
// implementation of read_username_from_file that has the same functionality
// but this implementation uses the ? operator
fn short_read_username_from_file() -> Result<String, io::Error> {
    // first use of the ? operator
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    /*
    If the value of the Result is an Ok, the value inside the Ok will get
    returned from this expression, and the program will continue. If the value
    is an Err, the Err will be returned from the whole function as if we had
    used the return keyword so the error value gets propagated to the calling code.

    There is a difference between what the match expression from
    read_username_from_file and the ? operator do: error values that have the
    ? operator called on them go through the from function, defined in the From
    trait in the standard library, which is used to convert errors from one type
    into another. When the ? operator calls the from function, the error type
    received is converted into the error type defined in the return type of the
    current function. This is useful when a function returns one error type to
    represent all the ways a function might fail, even if parts might fail for
    many different reasons. As long as each error type implements the from
    function to define how to convert itself to the returned error type, the ?
    operator takes care of the conversion automatically.

    In the context of short_read_username_from_file, the ? at the end of the
    File::open call will return the value inside an Ok to the variable f.
    If an error occurs, the ? operator will return early out of the whole
    function and give any Err value to the calling code. The same thing applies
    to the ? at the end of the read_to_string call.
    */
}

// you can even chain method calls immediately after the ? operator
fn chained_short_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// even shorter implementation using std::fs!
fn super_short_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
