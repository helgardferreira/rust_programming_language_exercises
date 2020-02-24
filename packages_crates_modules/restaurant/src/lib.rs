mod front_of_house;

// module tree for the above structure
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waiting_list
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// making use of the use keyword
// use crate::front_of_house::hosting;

// using relative path with use keyword
// use front_of_house::hosting;

// re-exporting item
// If we hadn't specified pub use, the eat_at_restaurant function could call
// hosting::add_to_waiting_list in its scope, but external code couldn't
// take advantage of this new path.
pub use crate::front_of_house::hosting;

mod back_of_house {
    // you choose which fields of a struct to make public - one at a time
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // when an enum is public all the variants become public (unlike structs)
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// idiomatic way to bring the standard library’s HashMap struct
// into the scope of a binary crate
use std::collections::HashMap;

// using nested paths
use std::{cmp::Ordering, io};

// using external packages
// bringing Rng trait into scope
use rand::Rng;

// the glob operator
// use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waiting_list();

    // Relative path
    // front_of_house::hosting::add_to_waiting_list();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);


    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}", order1);
    println!("{:?}", order2);

    // no longer need to specify lengthy path if using use keyword
    hosting::add_to_waiting_list();

    let mut map = HashMap::new();
    map.insert(1, 2);

    // using external packages
    // using thread_rng function on rand
    let secret_number = rand::thread_rng();
    println!("{:?}", secret_number);
}
