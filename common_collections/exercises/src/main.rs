use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
struct MMM {
    numbers: Vec<i32>,
    mean: Option<f32>,
    median: Option<f32>,
    mode: Option<i32>,
}

fn main() {
    exercise_one();
    exercise_two();
}

/*
Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position), and mode (the value that
occurs most often; a hash map will be helpful here) of the list.
*/
fn mmm_calc(numbers: &Vec<i32>) -> MMM {
    let mut result = MMM {
        numbers: numbers.clone(),
        mean: None,
        median: None,
        mode: None,
    };

    let mut mode_map: HashMap<&i32, i32> = HashMap::new();

    result.numbers.sort();

    let mut sum = 0;
    for number in result.numbers.iter() {
        sum += *number;
        *mode_map.entry(number).or_insert(0) += 1;
    }

    result.mean = Some(sum as f32 / result.numbers.len() as f32);
    result.median = if (result.numbers.len() + 1) % 2 == 0 {
        Some(result.numbers[(result.numbers.len() + 1) / 2 - 1] as f32)
    } else {
        Some(
            (result.numbers[result.numbers.len() / 2 - 1] as f32 +
                result.numbers[result.numbers.len() / 2] as f32) / 2.0
        )
    };

    // making HEAVY usage of how rust treats expressions
    // ... maybe too much?
    result.mode = {
        let mut max = 0;
        let mut max_key = 0;
        for (key, value) in mode_map {
            max = if value > max {
                max_key = *key;
                value
            } else {
                max
            }
        }
        Some(max_key)
    };

    result
}

fn exercise_one() {
    let foo = vec![1, 3, 2, 7, 5, 3, 10, 15];

    match mmm_calc(&foo) {
        MMM {
            numbers: _,
            mean: x,
            median: y,
            mode: z
        } => {
            match x {
                None => {}
                Some(val) => { println!("mean: {}", val) }
            }
            match y {
                None => {}
                Some(val) => { println!("median: {}", val) }
            }
            match z {
                None => {}
                Some(val) => { println!("mode: {}", val) }
            }
        }
    }
}

/*
Convert strings to pig latin. The first consonant of each word is moved to the
end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start
with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/
// utf-8 friendly-ish!
fn pig_latin(text: &str) -> String {
    // create new String to prevent modifying parameter's value
    let mut result = String::new();

    // create a collection of string literals to iterate through
    let words: Vec<&str> = text.split_whitespace().collect();

    for (index, word) in words.iter().enumerate() {
        // break word into characters
        let mut letters: Vec<char> = word.chars().collect();

        // pig-latin-ize
        // error handling to make sure word has any letters at all
        match letters.first() {
            None => {}
            Some(letter) => {
                // check if first letter is a consonant
                if *letter != 'a'
                    && *letter != 'e'
                    && *letter != 'i'
                    && *letter != 'o'
                    && *letter != 'u' {
                    // check if there is more than one letter or not
                    // this is to prevent unnecessary hyphens
                    let mut word_end =
                        if letters.len() > 1 {
                            vec!['-', letters.remove(0), 'a', 'y']
                        } else {
                            vec![letters.remove(0), 'a', 'y']
                        };
                    letters.append(&mut word_end);
                } else {
                    letters.append(&mut vec!['-', 'h', 'a', 'y'])
                }
            }
        }

        // push new pig-latin word to result string
        result.push_str(&String::from_iter(letters)[..]);

        // determine if space should be added or not
        if index != words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

fn exercise_two() {
    println!("{:?}", pig_latin("first tell me about your apples"));
    println!("{:?}", pig_latin("press f to pay respects"));
}
