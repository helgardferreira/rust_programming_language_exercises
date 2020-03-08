use std::fs;
use std::error::Error;

// be wary to not fall victim to 'primitive obsession'!
// making use of lifetime specifiers here since we're making use of references
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
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

        Ok(Config { query, filename })
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

    println!("With text:\n\n{}", contents);

    // if the function succeeds we (implicitly) return the Ok variant of Result
    Ok(())
}