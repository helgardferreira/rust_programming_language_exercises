use test_organization;

// submodules in integration tests
// take note of the directory structure
// tests -> common -> mod.rs
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}

/*
Integration Tests for Binary Crates

If our project is a binary crate that only contains a src/main.rs file and doesn't
have a src/lib.rs file, we can’t create integration tests in the tests directory
and bring functions defined in the src/main.rs file into scope with a use
statement. Only library crates expose functions that other crates can use; binary
crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a
straightforward src/main.rs file that calls logic that lives in the src/lib.rs
file. Using that structure, integration tests can test the library crate with use
to make the important functionality available. If the important functionality
works, the small amount of code in the src/main.rs file will work as well, and
that small amount of code doesn't need to be tested.
*/
