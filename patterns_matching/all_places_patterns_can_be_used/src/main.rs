fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // match example
    match favorite_color {
        None => println!("No favorite color!"),
        Some(color) => println!("Favorite color is: {}", color),
    }

    // conditional if let expressions
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color")
    }
    /*
    You can see that if let can also introduce shadowed variables in the same way
    that match arms can: the line if let Ok(age) = age introduces a new shadowed
    age variable that contains the value inside the Ok variant. This means we need
    to place the if age > 30 condition within that block: we can't combine these
    two conditions into if let Ok(age) = age && age > 30. The shadowed age we want
    to compare to 30 isn't valid until the new scope starts with the curly bracket.

    The downside of using if let expressions is that the compiler doesn't check
    exhaustiveness, whereas with match expressions it does. If we omitted the last
    else block and therefore missed handling some cases, the compiler would not
    alert us to the possible logic bug.
    */

    // while let conditional loops
    let mut stack: Vec<u8> = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops
    /*
    In a for loop, the pattern is the value that directly follows the keyword for,
    so in for x in y the x is the pattern.
    */
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    /*
    a let statement looks like this:

    let PATTERN = EXPRESSION;

    In statements like let x = 5; with a variable name in the PATTERN slot, the
    variable name is just a particularly simple form of a pattern. Rust compares
    the expression against the pattern and assigns any names it finds. So in the
    let x = 5; example, x is a pattern that means “bind what matches here to the
    variable x.” Because the name x is the whole pattern, this pattern effectively
    means “bind everything to the variable x, whatever the value is.”

    To see the pattern matching aspect of let more clearly, consider the below
    example, which uses a pattern with let to destructure a tuple.
    */
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);

    let point = (3, 5);
    print_coordinates(&point);
}

/*
Function Parameters - The below parameters are a pattern (which is used to
destructure a tuple)!
*/
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
