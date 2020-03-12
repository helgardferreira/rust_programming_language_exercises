/*
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access
some data at any given time. To access the data in a mutex, a thread must first signal that it
wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part
of the mutex that keeps track of who currently has exclusive access to the data.
Therefore, the mutex is described as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:

    - You must attempt to acquire the lock before using the data.
    - When you’re done with the data that the mutex guards, you must unlock the data so other
      threads can acquire the lock.
*/
use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // sharing a Mutex<T> between multiple threads
    /*let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }*/
    /*
    error[E0382]: use of moved value: `counter`
      --> src/main.rs:33:36
       |
    29 |     let counter = Mutex::new(0);
       |         ------- move occurs because `counter` has type `std::sync::Mutex<i32>`,
                         which does not implement the `Copy` trait
    ...
    33 |         let handle = thread::spawn(move || {
       |                                    ^^^^^^^ value moved into closure here, in previous
                                                    iteration of loop
    34 |             let mut num = counter.lock().unwrap();
       |                           ------- use occurs due to use in closure
    */

    /*let counter = Rc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }*/
    /*
    error[E0277]: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
       --> src/main.rs:58:22
        |
    58  |         let handle = thread::spawn(move || {
        |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent
                                             between threads safely
        |
        = help: within `[closure@src/main.rs:58:36: 62:10
          counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is not
          implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
        = note: required because it appears within the type `[closure@src/main.rs:58:36: 62:10
          counter:std::rc::Rc<std::sync::Mutex<i32>>]`
    */

    // atomic reference counting with Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// do research on deadlocks!