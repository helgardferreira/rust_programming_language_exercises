use super::*;

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn another() {
    panic!("Make this test fail");
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };

    assert!(!smaller.can_hold(&larger))
}

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

// adding custom failure messages
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}

// checking for panics with the should_panic annotation
#[test]
/*
Tests that use should_panic can be imprecise because they only indicate that the
code has caused some panic. A should_panic test would pass even if the test panics
for a different reason from the one we were expecting to happen.
To make should_panic tests more precise, we can add an optional expected parameter
to the should_panic attribute:
*/
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}

// using Result<T, E> in tests
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
