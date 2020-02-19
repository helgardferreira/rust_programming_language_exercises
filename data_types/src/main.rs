fn main() {
    // scalar types
    // integer types
    /*
    Length	    Signed	    Unsigned
    8-bit	    i8	u8
    16-bit	    i16	u16
    32-bit	    i32	u32
    64-bit	    i64	u64
    128-bit	    i128	    u128
    arch	    isize	    usize
    */
    // Signed numbers are stored using two’s complement representation.
    let x: i32 = 100;
    println!("{:?}", x);

    // Integer Overflow:
    // When you’re compiling in release mode with the --release flag
    // Rust does not include checks for integer overflow that cause panics.
    // Instead, if overflow occurs, Rust performs two’s complement wrapping.
    // In short, values greater than the maximum value the type can hold
    // “wrap around” to the minimum of the values the type can hold.
    // In the case of a u8, 256 becomes 0, 257 becomes 1, and so on.
    // The program won’t panic,but the variable will have a value
    // that probably isn’t what you were expecting it to have.
    // Relying on integer overflow’s wrapping behavior is considered an error.
    // If you want to wrap explicitly, you can use the standard library type Wrapping.

    // floating point types
    let x: f32 = 2.0; // f32
    let y: f64 = 3.0; // f64
    let z = 6.9; // f64 (The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision)

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    // boolean
    let t = true;
    let f: bool = false;

    println!("{:?}", t);
    println!("{:?}", f);

    // character type
    let c = 'z';
    let z = 'ℤ';
    let doggy_emoticon = '🐕';

    println!("{:?}", c);
    println!("{:?}", z);
    println!("{:?}", doggy_emoticon);

    // compound types
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = tup;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    // you can also access the individual members of a tuple like so
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{:?}", five_hundred);
    println!("{:?}", six_point_four);
    println!("{:?}", one);

    // array type
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // iterating through arrays with a for loop
    for month in months.iter() { println!("{:?}", month); }

    // explicitly stating the type of the array
    // arrays are of fixed type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for num in a.iter() { println!("{:?}", num); }

    let a = [3; 5]; // equivalent to let a = [3, 3, 3, 3, 3];
    for num in a.iter() { println!("{:?}", num); }
}
