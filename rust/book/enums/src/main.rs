
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn one() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn two() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

// Option enum
// Rust doesn't have "null"

// the concept of null is trying to express that the value is currently invalid or absent for some reason.
// rust has "Option" that can encode this null concept
// enum Option<T> {
//     None,
//     Some(T)
// }

fn three() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    // why is this better than just using null?
    // Option<T> and T are different types, and the compiler won't let us use Option<T> as if it were definitely a valid value

    // below will not compile:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    // you have to convert Option<T> into a T before doing anything.
}

fn main() {
    one();
    two();
}