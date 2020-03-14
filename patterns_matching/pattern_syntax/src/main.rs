fn main() {
    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    /*
    Keep in mind, that match starts a new scope. This means that any variables
    declared as part of a pattern inside the match expression will shadow those
    with the same name outside the match construct.

    This can be seen in the example below:
    */
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    /*
    N.B. Ranges are only allowed with numeric values or char values, because the
    compiler checks that the range isn't empty at compile time. The only types for
    which Rust can tell if a range is empty or not are char and numeric values.
    */

    // an example of a match range with char values
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    /*
    Destructuring to Break Apart Values

    Destructuring Structs
    */
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // shorthand for destructuring structs
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructure with literal values (useful for match expressions)
    match p {
        Point { x, y: 0 } => println!("Point lies on the x axis at {}", x),
        Point { x: 0, y } => println!("Point lies on the y axis at {}", y),
        Point { x, y } => println!(
            "Point lies on neither axis: ({}, {})",
            x,
            y,
        ),
    }

    /*
    Destructuring Enums
    */
    /*let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {}, and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            );
        }
    }*/

    /*
    Destructuring Nested Structs and Enums
    */
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => {}
    }

    /*
    Destructuring Structs and Tuples
    */
    let ((feet, inches), Point { x, y }) =
        ((3, 10), Point { x: 3, y: -10 });

    println!("Displacement in feet and inches: {}'{}\"", feet, inches);
    println!("Coordinates:\nx: {}\ny:{}", x, y);

    /*
    Ignoring Values in a Pattern

    Ignoring an Entire Value with _
    */

    foo(3, 4);

    /*
    Ignoring Parts of a Value with a Nested _
    */
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    /*
    Ignoring an Unused Variable by Starting Its Name with _
    */
    let _x = 5;
    let y = 10;
    /*
    warning: unused variable: `y`
       --> src\main.rs:186:9
        |
    186 |     let y = 10;
        |         ^ help: consider prefixing with an underscore: `_y`
        |
        = note: `#[warn(unused_variables)]` on by default
    */

    /*
    Ignoring Remaining Parts of a Value with ..
    */
    let origin = ThreeDimensionalPoint {
        x: 0,
        y: 0,
        z: 0,
    };

    match origin {
        ThreeDimensionalPoint { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    /*
    Extra Conditionals with Match Guards
    */
    let num = Some(4);

    match num {
        // match guard
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // using match guards to fix earlier example of variable shadowing in `match`
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => {
            println!("Matched, n = {}", n)
        }
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // the | operator with match guards
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }
    /*
    The match condition states that the arm only matches if the value of x is
    equal to 4, 5, or 6 and if y is true. When this code runs, the pattern of the
    first arm matches because x is 4, but the match guard if y is false, so the
    first arm is not chosen. The code moves on to the second arm, which does
    match, and this program prints no. The reason is that the if condition applies
    to the whole pattern 4 | 5 | 6, not only to the last value 6.
    */

    /*
    @ Bindings

    The at operator (@) lets us create a variable that holds a value at the same
    time we're testing that value to see whether it matches a pattern.
    */
    let msg = Greeting::Hello { id: 5 };

    match msg {
        // full syntax
        Greeting::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable);
        }
        /*
        N.B. we can't make use of the below id variable (and that's why we need
        the above @ operator).
        */
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range.");
        }
        // here we can make use of `id`, since we're making use of struct shorthand
        Greeting::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}

// ignoring an entire value with _ as a parameter
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

struct ThreeDimensionalPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Greeting {
    Hello { id: i32 },
}
