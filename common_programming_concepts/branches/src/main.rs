fn main() {
    // if expressions
    let number = 3;

    // condition MUST result in bool
    // else you'll get a compile error
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // handling multiple conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    // because if is an expression, we can use it
    // on the right side of a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // the result of the if expression must be consistent
    // else the compiler won't be able to accurately infer
    // the resulting type
    /*
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };
    */
    /*
      --> src/main.rs:45:9
       |
    42 |       let number = if condition {
       |  __________________-
    43 | |         5
       | |         - expected because of this
    44 | |     } else {
    45 | |         "six"
       | |         ^^^^^ expected integer, found `&str`
    46 | |     };
       | |_____- if and else have incompatible types
    */
}
