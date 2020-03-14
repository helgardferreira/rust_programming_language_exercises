fn main() {
    /*
    Patterns come in two forms: refutable and irrefutable. Patterns that will match
    for any possible value passed are irrefutable.

    Example:
    */
    let x = 5; // x matches for any possible value passed

    println!("{}", x);

    let some_option_value: Option<i32> = None;
    // the following code will not compile since let requires a irrefutable pattern
    /*let Some(x) = some_option_value;*/
    /*
    error[E0005]: refutable pattern in local binding: `None` not covered
      --> src\main.rs:12:9
       |
    12 |     let Some(x) = some_option_value;
       |         ^^^^^^^ pattern `None` not covered
       |
       = note: `let` bindings require an "irrefutable pattern", like a `struct` or
               an `enum` with only one variant
       = note: for more information, visit
               https://doc.rust-lang.org/book/ch18-02-refutability.html
    help: you might want to use `if let` to ignore the variant that isn't matched
       |
    12 |     if let Some(x) = some_option_value { /* */ }
       |
    */

    // we can change the code that uses the pattern - in this case using `if let`
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    /*
    However, if we attempt to assign this `if let` to a variable it will still not
    compile. This is because the expression as a whole is still refutable! To fix
    this we must add some additional behavior to ensure that all cases are being
    met (and thus the expression's pattern will be deemed irrefutable).
    */
    /*let x = if let Some(x) = some_option_value {
        x
    };*/
    /*
    error[E0317]: `if` may be missing an `else` clause
      --> src\main.rs:41:13
       |
    41 |       let x = if let Some(x) = some_option_value {
       |  _____________^
    42 | |         x
       | |         - found here
    43 | |     };
       | |_____^ expected `()`, found `i32`
       |
       = note: `if` expressions without `else` evaluate to `()`
       = help: consider adding an `else` block that evaluates to the expected type
    */

    let x = if let Some(x) = some_option_value {
        x
    } else {
        0
    };

    println!("{}", x);
}
