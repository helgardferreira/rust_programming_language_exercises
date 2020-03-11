/*
Programming languages implement threads in a few different ways. Many operating
systems provide an API for creating new threads. This model where a language calls
the operating system APIs to create threads is sometimes called 1:1, meaning one
operating system thread per one language thread.

Many programming languages provide their own special implementation of threads.
Programming language-provided threads are known as green threads, and languages
that use these green threads will execute them in the context of a different
number of operating system threads. For this reason, the green-threaded model is
called the M:N model: there are M green threads per N operating system threads,
where M and N are not necessarily the same number.

Each model has its own advantages and trade-offs, and the trade-off most important
to Rust is runtime support. Runtime is a confusing term and can have different
meanings in different contexts.

In this context, by runtime we mean code that is included by the language in every
binary. This code can be large or small depending on the language, but every
non-assembly language will have some amount of runtime code. For that reason,
colloquially when people say a language has "no runtime," they often mean
"small runtime." Smaller runtimes have fewer features but have the advantage of
resulting in smaller binaries, which make it easier to combine the language with
other languages in more contexts. Although many languages are okay with increasing
the runtime size in exchange for more features, Rust needs to have nearly no
runtime and cannot compromise on being able to call into C to maintain performance.

The green-threading M:N model requires a larger language runtime to manage threads.
As such, the Rust standard library only provides an implementation of 1:1 threading.
Because Rust is such a low-level language, there are crates that implement M:N
threading if you would rather trade overhead for aspects such as more control
over which threads run when and lower costs of context switching, for example.
*/
use std::thread;
// use std::time::Duration;

fn main() {
    /*thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }*/
    /*
    Outputs:

    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!

    As you can see, the spawned thread doesn't get the opportunity to finish. This
    is because in this scenario the main thread shuts down first.
    */

    /*let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();*/
    /*
    Outputs:

    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 4 from the main thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!

    The spawned thread is now allowed to finish - thanks to join!
    */

    // using move closures with threads
    /*let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();*/
    /*
    Without using `move`:

    error[E0373]: closure may outlive the current function, but it borrows `v`,
    which is owned by the current function
       --> src\main.rs:102:32
        |
    102 |     let handle = thread::spawn(|| {
        |                                ^^ may outlive borrowed value `v`
    103 |         println!("Here's a vector: {:?}", v);
        |                                           - `v` is borrowed here
        |
    note: function requires argument type to outlive `'static`
       --> src\main.rs:102:18
        |
    102 |       let handle = thread::spawn(|| {
        |  __________________^
    103 | |         println!("Here's a vector: {:?}", v);
    104 | |     });
        | |______^
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
        |
    102 |     let handle = thread::spawn(move || {
        |                                ^^^^^^^
    */

    /*
    By adding the move keyword we force the closure to take ownership of the
    values it's using rather than allowing Rust to infer that it should borrow
    the values.
    */
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
