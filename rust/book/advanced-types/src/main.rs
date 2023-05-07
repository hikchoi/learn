// type aliases
type Kilometers = i32;

// let f: Box<dyn Fn() + Send + 'static>

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {

// }

// fn returns_long_type -> Box<dyn Fn() + Send + 'static> {

// }

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {

}

use std::fmt;
use std::io::Error;
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// never type

fn bar() -> ! {

}

// ! can be coerced into any other type
// let guess = match guess.trim().parse() {
//     Ok(_) => 5, // u32
//     Err(_) => continue // !
// } // 'guess' is coerced to u32

// useful with panic! macro
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

// loops are expressions with ! type
    // print!("forever ");

    // loop {
    //     print!("and ever ");
    // }

// dynamically sized types and sized trait

// this is treated as:
// fn generic<T: ?Sized>(t: &T) {

// }
fn generic<T>(t: T) {

}



fn main() {
    println!("Hello, world!");
}
