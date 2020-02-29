use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for score in scores {
        println!("{} - {}", score.0, score.1);
    }

    // using zip to create a vector of tuples
    // and then using collect to turn that vector into a hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /*
    The type annotation HashMap<_, _> is needed here because it’s possible
    to collect into many different data structures and Rust doesn't know which you
    want unless you specify. For the parameters for the key and value types,
    however, we use underscores, and Rust can infer the types that the hash map
    contains based on the types of the data in the vectors.
    */

    for score in scores {
        println!("{} - {}", score.0, score.1);
    }

    // hash maps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // if you attempt to use them you'll get a similar error to the one bellow:
    /*
    error[E0382]: borrow of moved value: `field_name`
      --> src\main.rs:44:22
       |
    33 |     let field_name = String::from("Favorite color");
       |         ---------- move occurs because `field_name` has type `std::string::String`, which does
     not implement the `Copy` trait
    ...
    37 |     map.insert(field_name, field_value);
       |                ---------- value moved here
    ...
    44 |     println!("{:?}", field_name);
       |                      ^^^^^^^^^^ value borrowed here after move
    */

    // accessing values in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        None => { println!("Team not found!"); }
        Some(s) => { println!("{:?}", s); }
    };

    // iterating over each key/value pair in a hash map
    // N.B. this will print each pair in an arbitrary order!
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // updating a hash map
    // overwriting a value using insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // -> {"Blue": 25} : original value has been overwritten

    // only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry takes the key you want to check as a parameter
    // the return value of entry() is an enum called Entry
    // this represents a value that might or might not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // the following logic checks if a word is already in our hash map
    // and if it is it increments the count value by 1
    for word in text.split_whitespace() {
        // N.B. the or_insert method actually returns a mutable reference
        // to the value for this key
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    /*
    By default, HashMap uses a “cryptographically strong” hashing function that
    can provide resistance to Denial of Service (DoS) attacks. This is not the
    fastest hashing algorithm available, but the trade-off for better security
    that comes with the drop in performance is worth it. If you profile your
    code and find that the default hash function is too slow for your purposes,
    you can switch to another function by specifying a different hasher.
    A hasher is a type that implements the BuildHasher trait.
    */
}
