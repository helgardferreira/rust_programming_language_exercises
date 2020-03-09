/*
Specify the code to run when a value goes out of scope by implementing the Drop
trait. The Drop trait requires you to implement one method named drop that takes a
mutable reference to self. To see when Rust calls drop, let's implement drop with
println! statements for now.
*/
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // dropping a value early with std::mem::drop
    /*c.drop();*/
    /*
    error[E0040]: explicit use of destructor method
      --> src\main.rs:22:7
       |
    22 |     c.drop();
       |       ^^^^ explicit destructor calls not allowed
    */
    /*
    Rust doesn't let us call drop explicitly because Rust would still
    automatically call drop on the value at the end of main. This would be
    a double free error because Rust would be trying to clean up the same
    value twice.

    We can't disable the automatic insertion of drop when a value goes out of
    scope, and we can't call the drop method explicitly. So, if we need to force
    a value to be cleaned up early, we can use the std::mem::drop function.

    The std::mem::drop function is different from the drop method in the Drop
    trait. We call it by passing the value we want to force to be dropped early
    as an argument.
    */
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
