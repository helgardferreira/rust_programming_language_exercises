use std::fs;
use std::error::Error;
use std::env;

// be wary to not fall victim to 'primitive obsession'!
// making use of lifetime specifiers here since we're making use of references
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

// we will rather opt to change the below function into a 'new' method for Config
// this helps in making our code more idiomatic
/*fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];

    Config {
        query,
        filename,
    }
}*/

// parse_config function that makes use of .clone() for simplicity
/*
But then you're left with the memory inefficiency and runtime performance
trade-offs inherent to the use of the clone method
*/
/*fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query,
        filename,
    }
}*/

/*
Here we must either explicitly state the lifetime of our implementation or explicitly
mark that our lifetime is elided with the anonymous lifetime specifier `<'_>`,
since the compiler can't implicitly determine if our lifetimes are elided
in this context.
*/
impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // return the Err variant of the Result enum and pass it a string literal
        if args.len() < 3 {
            return Err("Not enough arguments provided!");
        }

        let query = &args[1];
        let filename = &args[2];

        /*
        is_err is a method on Result that returns a boolean that is either true
        (if the value is the Err() variant) or false (if the value is the Ok()
        variant).

        Therefore, if not CASE_INSENSITIVE (i.e. the result is an error) then make a
        case sensitive search.

        Please note, in this case we don't care for the value of the environment
        variable - all we care for is whether it is set or not.
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

/*
Box<dyn Error> means the function will return a type that implements the
Error trait, but we don't have to specify what particular type the return value
will be.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*
    We make use of the ? operator to propagate any relevant errors from run to
    whatever is invoking run (in this case main). ? will return immediately if
    there is an error as well as pass along the Err(e) to the outer function.
    */
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // if the function succeeds we (implicitly) return the Ok variant of Result
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    // iterate through each line of the contents
    for line in contents.lines() {
        // check whether the line contains our query string
        if line.contains(query) {
            // store line in results vector
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    // iterate through each line of the contents
    /*
    We can't make use contents.to_lowercase() or contents.to_ascii_lowercase()
    since the values in the results vector will then reference data that is
    owned by the current function instead of data that is owned by the parameter
    (since both .to_lowercase() and .to_ascii_lowercase() return a String and
    not a &str).
    */
    for line in contents.lines() {
        // check whether the line contains our query string
        if line.to_lowercase().contains(&query) {
            // store line in the results vector
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape.";

        /*
        Takes a query and the text for the query and returns the lines from
        the text that contain the query.
        */
        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
