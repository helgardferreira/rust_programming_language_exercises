// adding useful functionality with derived traits
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area(rect1)
    );

    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect2)
    );

    println!("rect1 is {:#?}", rect2);
}

// the issue with this code is evident in the signature of area
/*
The area function is supposed to calculate the area of one rectangle,
but the function we wrote has two parameters. The parameters are related,
but that’s not expressed anywhere in our program. It would be more readable
and more manageable to group width and height together.
*/
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refactoring with tuples
/*
In one way, this program is better. Tuples let us add a bit of structure,
and we’re now passing just one argument. But in another way, this version is
less clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.

It doesn't matter if we mix up width and height for the area calculation,
but if we want to draw the rectangle on the screen, it would matter! We would
have to keep in mind that width is the tuple index 0 and height is the tuple
index 1. If someone else worked on this code, they would have to figure this out
and keep it in mind as well. It would be easy to forget or mix up these values
and cause errors, because we haven’t conveyed the meaning of our data in our code.
*/
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// refactoring with structs
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
