// we can also define structs to use a generic type parameter
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MultipleTypePoint<T, U> {
    x: T,
    y: U,
}

// ... and with enums!
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MultipleTypePoint<T, U> {
    // a method that uses different generic types from its struct’s definition
    fn mix_up<V, W>(
        self,
        other: MultipleTypePoint<V, W>,
    ) -> MultipleTypePoint<T, W> {
        MultipleTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);

    // don't mismatch types with a singular generic parameter!
    /*let wont_work = Point { x: 5, y: 4.0 };*/
    /*
    error[E0308]: mismatched types
      --> src\main.rs:31:38
       |
    31 |     let wont_work = Point { x: 5, y: 4.0 };
       |                                      ^^^ expected integer, found floating-point number
    */

    let both_integer = MultipleTypePoint { x: 5, y: 10 };
    let both_float = MultipleTypePoint { x: 1.0, y: 4.0 };
    let integer_and_float = MultipleTypePoint { x: 5, y: 4.0 };
    println!("{:?}", both_integer);
    println!("{:?}", both_float);
    println!("{:?}", integer_and_float);

    let p1 = Point { x: 5, y: 10 };
    let p2: Point<f32> = Point { x: 10.0, y: 0.0 };

    println!("{:?}", p1.x);
    println!("{:?}", p2.distance_from_origin());

    let p3 = MultipleTypePoint { x: 5, y: 10.4 };
    let p4 = MultipleTypePoint { x: "Hello", y: 'c' };

    let p5 = p3.mix_up(p4);

    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// function to find largest signed 32-bit integer in a slice
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function to find the largest character in a slice
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
Let's eliminate the duplication by introducing a generic type parameter
in a single function.
To define the generic largest function, place type name declarations inside
angle brackets, <>, between the name of the function and the parameter list,
like this:
*/
// we read this definition as: the function largest is generic over some type T
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// fixing the largest function with trait bounds
/*
First we modify the signature of largest to be bound to PartialOrd. But then we
get the following error:

error[E0508]: cannot move out of type `[T]`, a non-copy slice
   --> src\main.rs:154:23
    |
154 |     let mut largest = list[0];
    |                       ^^^^^^^
    |                       |
    |                       cannot move out of here
    |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
    |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
   --> src\main.rs:156:18
    |
156 |     for &item in list.iter() {
    |         -----    ^^^^^^^^^^^
    |         ||
    |         |data moved here
    |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
    |         help: consider removing the `&`: `item`

When we made the largest function generic, it became possible for the list
parameter to have types in it that don't implement the Copy trait. Consequently,
we wouldn't be able to move the value out of list[0] and into the largest
variable, resulting in this error.

To call this code with only those types that implement the Copy trait,
we can add Copy to the trait bounds of T!
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
