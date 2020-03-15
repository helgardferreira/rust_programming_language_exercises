use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );

    // lexical scope with closures (capturing the environment of closures)
    let x = 4;

    // demonstrating that the closure has access to its lexical scope
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    /*
    Closures can capture values from their environment in three ways, which
    directly map to the three ways a function can take a parameter: taking
    ownership, borrowing mutably, and borrowing immutably. These are encoded in
    the three Fn traits as follows:

    -   FnOnce consumes the variables it captures from its enclosing scope,
        known as the closure’s environment. To consume the captured variables,
        the closure must take ownership of these variables and move them into the
        closure when it is defined. The Once part of the name represents the fact
        that the closure can’t take ownership of the same variables more than
        once, so it can be called only once.
    -   FnMut can change the environment because it mutably borrows values.
    -   Fn borrows values from the environment immutably.
    */

    // how to force closure to take ownership of the values it uses:
    let x = vec![1, 2, 3];

    // making use of the `move` keyword
    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// simulated hypothetical example
/*fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}*/

// storing closures using generic parameters and the Fn traits
// the Cacher struct, below, holds a closure and an optional result value
// making use of the Fn trait allows us to 'capture' from the function environment
struct Cacher<T, K, V>
    where
        K: Copy + Eq + Hash,
        V: Copy,
        T: Fn(K) -> V
{
    calculation: T,
    // we make use of a HashMap to cache multiple values and their arguments
    values: HashMap<K, V>,
}

// memoization / lazy-loading method implementation for the Cacher struct
impl<T, K, V> Cacher<T, K, V>
    where
        K: Copy + Eq + Hash,
        V: Copy,
        T: Fn(K) -> V
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    // only invoke the calculation closure function if there is no (None) value
    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // refactoring with closures to store code
    // even though it's not necessary to annotate the type of a closure, you can:
    // let expensive_closure = |num: u32| -> u32 {
    /*let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };*/

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    /*
    Visual representation of the similarities / dissimilarities of closures and
    functions:

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    */

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_result,
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            // expensive_result,
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                // expensive_result,
                expensive_result.value(intensity)
            );
        }
    }
}
