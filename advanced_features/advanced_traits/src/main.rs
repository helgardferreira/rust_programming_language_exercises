/*
Specifying Placeholder Types in Trait Definitions with Associated Types
*/
/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}*/
/*
The type Item is a placeholder type, and the next method's definition shows that it
will return values of type Option<Self::Item>. Implementors of the Iterator trait
will specify the concrete type for Item, and the next method will return an Option
containing a value of that concrete type.

Associated types might seem like a similar concept to generics, in that the latter
allow us to define a function without specifying what types it can handle. So why
use associated types?

Let's examine the difference between the two concepts with the following example
that implements the Iterator trait on the Counter struct.
*/
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
/*
This syntax seems comparable to that of generics. So why not just define the
Iterator trait with generics, as shown below?
*/
/*pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}*/
/*
The difference is that when using generics, as in above, we must annotate the
types in each implementation; because we can also implement Iterator<String> for
Counter or any other type, we could have multiple implementations of Iterator for
Counter. In other words, when a trait has a generic parameter, it can be
implemented for a type multiple times, changing the concrete types of the generic
type parameters each time. When we use the next method on Counter, we would have to
provide type annotations to indicate which implementation of Iterator we want to
use.

With associated types, we don't need to annotate types because we can't implement
a trait on a type multiple times. We can only choose what the type of Item will be
once, because there can only be one impl Iterator for Counter. We don't have to
specify that we want an iterator of u32 values everywhere that we call next on
Counter.
*/

/*
Default Generic Type Parameters and Operator Overloading
*/
/*
Rust doesn't allow you to create your own operators or overload arbitrary
operators. But you can overload the operations and corresponding traits listed in
std::ops by implementing the traits associated with the operator. For example,
below we overload the + operator to add two Point instances together. We do this
by implementing the Add trait on a Point struct:
*/
use std::ops::{Add, Deref};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
/*
The default generic type in this code is within the Add trait. Here is its
definition:
*/
/*trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}*/
/*
When we implemented Add for Point, we used the default for RHS because we wanted
to add two Point instances. Let's look at an example of implementing the Add trait
where we want to customize the RHS type rather than using the default.

We have two structs, Millimeters and Meters, holding values in different units.
We want to add values in millimeters to values in meters and have the
implementation of Add do the conversion correctly. We can implement Add for
Millimeters with Meters as the RHS, as shown below.
*/
// both Millimeters and Meters below are tuple structs
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/*
Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as
another trait's method, nor does Rust prevent you from implementing both traits
on one type. It's also possible to implement a method directly on the type with
the same name as methods from traits.

When calling methods with the same name, you'll need to tell Rust which one you
want to use. Consider the code below where we've defined two traits, Pilot and
Wizard, that both have a method called fly. We then implement both traits on a
type Human that already has a method named fly implemented on it. Each fly method
does something different.
*/
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/*
Because the fly method takes a self parameter, if we had two types that both
implement one trait, Rust could figure out which implementation of a trait to use
based on the type of self.

However, associated functions that are part of traits don't have a self parameter.
When two types in the same scope implement that trait, Rust can't figure out which
type you mean unless you use fully qualified syntax. For example, the Animal trait
in the below example has the associated function baby_name, the implementation of
Animal for the struct Dog, and the associated function baby_name defined on Dog
directly.
*/
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/*
Using Supertraits to Require One Trait's Functionality Within Another Trait

Sometimes, you might need one trait to use another trait's functionality. In this
case, you need to rely on the dependent trait also being implemented. The trait
you rely on is a supertrait of the trait you're implementing.
*/
use std::fmt;
use std::fmt::Formatter;

/*
In the implementation of outline_print, we want to use the Display trait's
functionality. Therefore, we need to specify that the OutlinePrint trait will work
only for types that also implement Display and provide the functionality that
OutlinePrint needs. We can do that in the trait definition by specifying
OutlinePrint: Display. This technique is similar to adding a trait bound to the
trait.
*/
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        /*
        Because we've specified that OutlinePrint requires the Display trait, we
        can use the to_string function that is automatically implemented for any
        type that implements Display.
        */
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

/*
Let's see what happens when we try to implement OutlinePrint on a type that
doesn't implement Display, such as the Point struct:
*/
/*impl OutlinePrint for Point {}*/
/*
error[E0277]: `Point` doesn't implement `std::fmt::Display`
       --> src\main.rs:235:6
        |
    235 | impl OutlinePrint for Point {}
        |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter
        |
        = help: the trait `std::fmt::Display` is not implemented for `Point`
        = note: in format strings you may be able to use `{:?}` (or {:#?} for
          pretty-print) instead
*/
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

/*
Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the "Implementing a Trait on a Type" section, we mentioned the
orphan rule that states we're allowed to implement a trait on a type as long as
either the trait or the type are local to our crate. It's possible to get around
this restriction using the newtype pattern, which involves creating a new type in
a tuple struct. The tuple struct will have one field and be a thin wrapper around
the type we want to implement a trait for. Then the wrapper type is local to our
crate, and we can implement the trait on the wrapper. Newtype is a term that
originates from the Haskell programming language. There is no runtime performance
penalty for using this pattern, and the wrapper type is elided at compile time.

As an example, let's say we want to implement Display on Vec<T>, which the orphan
rule prevents us from doing directly because the Display trait and the Vec<T>
type are defined outside our crate. We can make a Wrapper struct that holds an
instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T>
value, as shown below:
*/

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // explicit type annotation for interest's sake
        // let x: Result<(), Error> = write!(f, "[{}]", self.0.join(", "));

        // destructuring tuple struct for interest's sake
        // let Wrapper(string_collection) = self;
        // write!(f, "[{}]", string_collection.join(", "))

        write!(f, "[{}]", self.0.join(", "))
    }
}

/*
The downside of using this technique is that Wrapper is a new type, so it doesn't
have the methods of the value it's holding. We would have to implement all the
methods of Vec<T> directly on Wrapper such that the methods delegate to self.0,
which would allow us to treat Wrapper exactly like a Vec<T>. If we wanted the new
type to have every method the inner type has, implementing the Deref trait
(discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References
with the Deref Trait” section) on the Wrapper to return the inner type would be a
solution. If we don't want the Wrapper type to have all the methods of the inner
type—for example, to restrict the Wrapper type's behavior—we would have to
implement just the methods we do want manually.

Like below:
*/

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        // using tuple struct destructuring
        /*let Wrapper(r) = self;
        r*/
        &self.0
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    let mm = Millimeters(123) + Meters(2);

    println!("{:?}", mm);

    /*
    Explicitly and implicitly invoking the fly method on the type itself:
    */
    let person = Human;
    person.fly();
    Human::fly(&person);
    /*
    Explicitly invoking the fly method on the different traits implemented for
    the type:
    */
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    /*
    We call the Dog::baby_name function, which calls the associated function
    defined on Dog directly. This code prints the following:

    A baby dog is called a Spot

    This output isn't what we wanted. We want to call the baby_name function that
    is part of the Animal trait that we implemented on Dog so the code prints A
    baby dog is called a puppy. The technique of specifying the trait name that we
    used previously, in the fly example, doesn't help here; the following will
    give us a compilation error.
    */
    /*println!("A baby dog is called a {}", Animal::baby_name())*/
    /*
    error[E0283]: type annotations needed
       --> src\main.rs:235:43
        |
    180 |     fn baby_name() -> String;
        |     ------------------------- required by `Animal::baby_name`
    ...
    235 |     println!("A baby dog is called a {}", Animal::baby_name())
        |                                           ^^^^^^^^^^^^^^^^^ cannot infer
                                                                      type
        |
        = note: cannot resolve `_: Animal`
    */
    /*
    To disambiguate and tell Rust that we want to use the implementation of
    Animal for Dog, we need to use fully qualified syntax. The below example
    demonstrates how to use fully qualified syntax.
    */
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let point = Point { x: 1, y: 2 };
    point.outline_print();

    let string_collection = vec![
        String::from("hello"), String::from("world")
    ];

    println!("{:?}", string_collection.len());

    let w = Wrapper(string_collection);
    println!("w = {}", w);

    let x = w.len();
    println!("{:?}", x);
}
