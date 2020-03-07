// public function
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// private function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/*
The attribute cfg stands for configuration and tells Rust that the following item
should only be included given a certain configuration option. In this case, the
configuration option is test, which is provided by Rust for compiling and running
tests. By using the cfg attribute, Cargo compiles our test code only if we actively
run the tests with cargo test. This includes any helper functions that might be
within this module, in addition to the functions annotated with #[test].
*/
#[cfg(test)]
mod tests {
    use super::*;

    // testing private functions
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
