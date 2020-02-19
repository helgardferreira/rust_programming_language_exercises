// constant variable
// can never be mutable
// is declared with const keyword
// has to have type specified
const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // the following will fail
    // since variables are immutable by default in rust
    /*
    x = 6;
    println!("The value of x is: {}", x);
    */
    /*
     --> src/main.rs:4:5
      |
    2 |     let x = 5;
      |         -
      |         |
      |         first assignment to `x`
      |         help: make this binding mutable: `mut x`
    3 |     println!("The value of x is: {}", x);
    4 |     x = 6;
      |     ^^^^^ cannot assign twice to immutable variable
    */

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!(
        "The value of constant MAX_POINTS is: {}",
        MAX_POINTS
    );

    // variable shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    // with variable shadowing you can:
    // shadow both the variables value AND its type
    // you can NOT do the same with mutable variables
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    */
    /*
    error[E0308]: mismatched types
      --> src/main.rs:50:14
       |
    50 |     spaces = spaces.len();
       |              ^^^^^^^^^^^^ expected `&str`, found `usize`
    */
}
