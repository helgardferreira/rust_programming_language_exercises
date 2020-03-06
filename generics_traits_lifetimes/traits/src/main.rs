/*
Note: Traits are similar to a feature often called interfaces in other languages,
although with some differences.
*/

/*
A type’s behavior consists of the methods we can call on that type.
Different types share the same behavior if we can call the same methods on all of
those types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a NewsArticle struct that holds a news story filed in a particular
location and a Tweet that can have at most 280 characters along with metadata that
indicates whether it was a new tweet, a re-tweet, or a reply to another tweet.

We want to make a media aggregator library that can display summaries of data that
might be stored in a NewsArticle or Tweet instance. To do this, we need a summary
from each type, and we need to request that summary by calling a summarize method
on an instance.

Below is the definition of a Summary trait that expresses this behaviour:
*/
use std::fmt::{Debug, Display};

pub trait Summary {
    /*fn summarize(&self) -> String;*/
    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait on a type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub re_tweet: bool,
}

// implementing a trait on a type
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[derive(Debug)]
pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

// implementing a trait on a type
impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("The Great {}", self.author)
    }
}

// using trait bounds to conditionally implement methods
/*
By using a trait bound with an impl block that uses generic type parameters, we
can implement methods conditionally for types that implement the specified traits.
For example, the type Pair<T> in below always implements the new function.
But Pair<T> only implements the cmp_display method if its inner type T implements
the PartialOrd trait that enables comparison and the Display trait that enables
printing.
*/
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

/*
We can also conditionally implement a trait for any type that implements
another trait (blanket implementations):
*/
// signature of the standard library's ToString trait
/*impl <T: Display> ToString for T {
    // --snip--
}*/

/*
Because the standard library has this blanket implementation, we can call the
to_string method defined by the ToString trait on any type that implements the
Display trait. For example, we can turn integers into their corresponding String
values like this because integers implement Display:
*/
// let s = 3.to_string();

fn main() {
    let tweet = Tweet {
        username: String::from("horse_e_books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        re_tweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    let post = BlogPost {
        title: String::from("I'm So Sick of Load-shedding"),
        author: String::from("John Doe"),
        content: String::from("Enough is enough! It's time to take action..."),
    };

    println!("New article available!\n{}", article.summarize());
    println!("1 new tweet:\n{}", tweet.summarize());
    println!("New blog-post available!\n{}", post.summarize());

    notify(&article);
    trait_bound_notify(&article);
    debug_notify(&post);
}

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// trait bound syntax
fn trait_bound_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// specifying multiple trait bounds with the + syntax
fn debug_notify(item: &(impl Summary + Debug)) {
    println!("{:?}", item);
}

// clearer trait bounds with where clauses
// instead of writing this:
/*fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {*/
// we can use a where clause:
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ 0 }

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        re_tweet: false,
    }
}

/*
You can only use impl Trait if you’re returning a single type. For example,
this code that returns either a NewsArticle or a Tweet with the return type
specified as impl Summary wouldn't work:
*/
/*fn returns_summarizables(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            re_tweet: false,
        }
    }
}*/
/*
Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions
around how the impl Trait syntax is implemented in the compiler.

Below is the error message given by the compiler for the above code snippet:
*/
/*
error[E0308]: `if` and `else` have incompatible types
   --> src\main.rs:162:9
    |
153 |   /     if switch {
154 |   |         NewsArticle {
    |  _|_________-
155 | | |             headline: String::from("Penguins win the Stanley Cup Championship!"),
156 | | |             location: String::from("Pittsburgh, PA, USA"),
157 | | |             author: String::from("Iceburgh"),
158 | | |             content: String::from("The Pittsburgh Penguins once again are the best
159 | | |             hockey team in the NHL."),
160 | | |         }
    | |_|_________- expected because of this
161 |   |     } else {
162 | / |         Tweet {
163 | | |             username: String::from("horse_ebooks"),
164 | | |             content: String::from("of course, as you probably already know, people"),
165 | | |             reply: false,
166 | | |             re_tweet: false,
167 | | |         }
    | |_|_________^ expected struct `NewsArticle`, found struct `Tweet`
168 |   |     }
    |   |_____- `if` and `else` have incompatible types
*/
