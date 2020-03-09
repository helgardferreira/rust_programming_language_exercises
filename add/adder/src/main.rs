use add_one;
use add_two;

fn main() {
    let num1 = 10;
    let num2 = 14;

    println!("{} plus one is {}", num1, add_one::add_one(num1));
    println!("{} plus two is {}", num2, add_two::add_two(num2));
}
