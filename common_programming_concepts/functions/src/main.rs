fn main() {
    println!("Hello, world");

    // invoking / calling a function
    another_function();
    function_with_params(5, 6);

    // statements vs expressions
    // statements are instructions that perform some action and do not return a value
    // expressions evaluate to a resulting value

    // statement
    let y = 6;

    println!("{:?}", y);

    // cannot be used as an expression
    /*
    let x = (let y = 6);
    */
    /*
    error: expected expression, found statement (`let`)
      --> src/main.rs:15:14
       |
    15 |     let x = (let y = 6);
       |              ^^^^^^^^^
       |
       = note: variable declaration using `let` is a statement
    */

    // expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// defining a function
fn another_function() {
    println!("Another function.");
}

fn function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// function with return values
// we make use of -> to define the output type of a function
fn five() -> i32 {
    5
}

// function with return type and parameter
fn plus_one(x: i32) -> i32 {
    x + 1
}
