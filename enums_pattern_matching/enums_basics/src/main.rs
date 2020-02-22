enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum ConciseIpAddrKind {
    V4(String),
    V6(String),
}

enum DifferingTypeIpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct equivalents to Message enum
/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

// defining a method on an enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Option enum
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", home.address);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", loopback.address);

    let home = ConciseIpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = ConciseIpAddrKind::V6(String::from("::1"));

    let home = DifferingTypeIpAddrKind::V4(127, 0, 0, 1);

    let loopback = DifferingTypeIpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
