//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
use std::error::Error;

/// Runs the library code.
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello World!");
    Ok(())
}

/*
Commonly Used Sections

We used the # Examples Markdown heading below to create a section in the HTML with
the title “Examples.” Here are some other sections that crate authors commonly use
in their documentation:

    =   Panics: The scenarios in which the function being documented could panic.
        Callers of the function who don’t want their programs to panic should make
        sure they don’t call the function in these situations.
    =   Errors: If the function returns a Result, describing the kinds of errors
        that might occur and what conditions might cause those errors to be
        returned can be helpful to callers so they can write code to handle the
        different kinds of errors in different ways.
    =   Safety: If the function is unsafe to call, there should be a section explaining why the function is
        unsafe and covering the invariants that the function expects callers to
        uphold.
*/

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn adds_one() {
        let arg = 5;
        let answer = add_one(arg);

        assert_eq!(6, answer);
    }
}
