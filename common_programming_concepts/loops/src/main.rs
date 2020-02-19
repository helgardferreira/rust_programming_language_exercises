fn main() {
    // infinite loop
    /*
    loop {
        println!("again!");
    }
    */

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can return a value from a loop
            // if you place an expression after a break statement
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}...", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //looping through a collection with for
    let a = [10, 20, 30, 40, 50];

    // less prone to error than a while loop
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // making use of for loop and Range
    for number in (1..4).rev() {
        println!("{:?}...", number);
    }
    println!("LIFTOFF!!!");
}
