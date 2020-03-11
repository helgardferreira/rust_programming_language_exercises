/*
One major tool Rust has for accomplishing message-sending concurrency is the
channel, a programming concept that Rust's standard library provides an
implementation of. You can imagine a channel in programming as being like a channel
of water, such as a stream or a river. If you put something like a rubber duck or
boat into a stream, it will travel downstream to the end of the waterway.

A channel in programming has two halves: a transmitter and a receiver. The
transmitter half is the upstream location where you put rubber ducks into the river,
and the receiver half is where the rubber duck ends up downstream. One part of
your code calls methods on the transmitter with the data you want to send, and
another part checks the receiving end for arriving messages. A channel is said to
be closed if either the transmitter or receiver half is dropped.
*/
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // creating a channel
    let (tx, rx) = mpsc::channel();

    // have the transmitting end send one string to the main thread
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        /*
        The following line will not allow the application to compile. This is
        because we are moving val's ownership from the closure to the send method.
        This is great because if Rust allowed this then once the value has been
        sent to another thread, that thread could modify or drop the value before
        we try to use the value again. This could lead to some  or other error.
        Thanks to Rust the error is now prevented at compile-time rather than
        actualizing during run-time.
        */
        // println!("val is {}", val);
    });

    // attempt to receive transmitted value
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // wait for thread to finish before moving on
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    // spawned thread now sends multiple messages and waits 1 second between each
    let handle = thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // instead of invoking recv() we are now treating rx like an iterator
    for received in rx {
        println!("Got: {}", received);
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let values = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    /*
    Output:

    Got: hello
    Got: more
    Got: from
    Got: messages
    Got: the
    Got: for
    Got: you
    Got: thread

    This is not the intuitive output, and as such you can see -
    concurrency can be tricky!
    */
}

