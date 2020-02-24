// using an enum to store multiple types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vector type annotation
    // let v: Vec<i32> = Vec::new();

    // using the vector macro
    let mut v = vec![1, 2, 3, 4, 5];

    // updating a vector
    v.push(5);
    println!("{:?}", v);

    // reading elements of vectors
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    /*let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);*/

    // ownership rules apply in the same ways to vectors
    /*let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);*/

    /*
    error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
      --> src/main.rs:31:5
       |
    29 |     let first = &v[0];
       |                  - immutable borrow occurs here
    30 |
    31 |     v.push(6);
       |     ^^^^^^^^^ mutable borrow occurs here
    32 |
    33 |     println!("The first element is: {}", first);
       |                                          ----- immutable borrow later used here
    */

    // iterating over the values in a vector
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        // (*) is the dereference operator
        *i += 50;
        println!("{}", i);
    }

    // using an enum with vectors
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        match i {
            SpreadsheetCell::Int(x) => println!("{}", x),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(t) => println!("{}", t),
        }
    }

    // dropping a vector drops its elements
}   // <- v goes out of scope and its freed here
