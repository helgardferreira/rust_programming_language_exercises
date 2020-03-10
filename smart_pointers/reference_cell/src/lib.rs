pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(
                "Urgent warning: You've used up over 90% of your quota!"
            );
        } else if percentage_of_max >= 0.75 {
            self.messenger.send(
                "Warning: You've used up over 75% of your quota!"
            );
        }
    }
}

/*
One important part of this code is that the Messenger trait has one method called
send that takes an immutable reference to self and the text of the message. This
is the interface our mock object needs to have. The other important part is that
we want to test the behavior of the set_value method on the LimitTracker. We can
change what we pass in for the value parameter, but set_value doesn't return
anything for us to make assertions on. We want to be able to say that if we create
a LimitTracker with something that implements the Messenger trait and a particular
value for max, when we pass different numbers for value, the messenger is told to
send the appropriate messages.

We need a mock object that, instead of sending an email or text message when we
call send, will only keep track of the messages it's told to send. We can create
a new instance of the mock object, create a LimitTracker that uses the mock object,
call the set_value method on LimitTracker, and then check that the mock object has
the messages we expect.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // collection of owned strings
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        /*
        We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get
        a mutable reference to the value inside the RefCell<Vec<String>>, which is
        the vector. Then we can call push on the mutable reference to the vector to
        keep track of the messages sent during the test.
        */
        fn send(&self, message: &str) {
            // .borrow_mut() returns the smart pointer type RefMut<T>
            self.sent_messages.borrow_mut().push(String::from(message));

            /*let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));*/
            /*
            thread 'tests::it_sends_an_over_75_percent_warning_message' panicked
            at 'already borrowed: BorrowMutError',
            /rustc/75cf41afb468152611212271bae026948cd3ba46\src\libcore\cell.rs:878:9
            */
        }
        // error without using RefCell<T>:
        /*
        error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
          --> src\lib.rs:77:13
           |
        76 |         fn send(&self, message: &str) {
           |                 ----- help: consider changing this to be a mutable
                                         reference: `&mut self`
        77 |             self.sent_messages.push(String::from(message));
           |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data
                                            it refers to cannot be borrowed as mutable
        */
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100,
        );

        limit_tracker.set_value(80);

        // .borrow() returns the smart pointer type Ref<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
/*
Catching borrowing errors at runtime rather than compile time means that you
would find a mistake in your code later in the development process and possibly
not until your code was deployed to production. Also, your code would incur a
small runtime performance penalty as a result of keeping track of the borrows at
runtime rather than compile time. However, using RefCell<T> makes it possible to
write a mock object that can modify itself to keep track of the messages it has
seen while you're using it in a context where only immutable values are allowed.
You can use RefCell<T> despite its trade-offs to get more functionality than
regular references provide.
*/
