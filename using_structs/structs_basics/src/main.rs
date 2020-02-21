// defining a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // creating an instance of our User struct
    let mut user1 = User {
        email: String::from("hrfer3@gmail.com"),
        username: String::from("helgard_f"),
        active: true,
        sign_in_count: 1,
    };

    // accessing a value from a struct
    println!("{:?}", user1.email);

    // changing a value in a struct
    // only works if the entire instance is mutable!
    user1.email = String::from("richardfart@gmail.com");
    println!("{:?}", user1.email);

    let user2 = build_user(
        String::from("john@doe.com"),
        String::from("john.doe"),
    );

    println!("{} - {}", user2.username, user2.email);

    // creating instances from other instances with struct update syntax
    let user3 = User {
        email: String::from("venessamk2@gmail.com"),
        username: String::from("venessamk2"),
        ..user1
    };
    println!("{:?}", user3.email);

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black.0);
    println!("{:?}", origin.0);

    // ownership of struct data
    /*
    In the User struct definition in Listing 5-1, we used the owned String type
    rather than the &str string slice type. This is a deliberate choice because
    we want instances of this struct to own all of its data and for that data to
    be valid for as long as the entire struct is valid.

    It’s possible for structs to store references to data owned by something else,
    but to do so requires the use of lifetimes, a Rust feature that we’ll discuss
    in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid
    for as long as the struct is. Let’s say you try to store a reference in a
    struct without specifying lifetimes, like this, which won’t work:
    */
    /*
    struct OwnerUser {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    };

    let user4 = OwnerUser {
        email: "jane@doe.com",
        username: "jane.doe",
        active: true,
        sign_in_count: 1,
    };
    */
    /*
    error[E0106]: missing lifetime specifier
      --> src/main.rs:52:19
       |
    52 |         username: &str,
       |                   ^ expected lifetime parameter

    error[E0106]: missing lifetime specifier
      --> src/main.rs:53:16
       |
    53 |         email: &str,
       |                ^ expected lifetime parameter
    */
}

// returning an instance of a struct implicitly using Rust's expressions behavior
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
