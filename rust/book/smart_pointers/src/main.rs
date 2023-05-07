
// interacting with values stored in Box<T>
fn one() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// recursive types: cons-list
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn two() {
    use crate::List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// folowing the pointer to the value
fn three() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Using Box<T> like a reference
fn four() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn five() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// implicit deref coercions with funcitons and methods
fn hello(name: &str) {
    println!("Hello, {name}");
}

fn six() {
    let me = MyBox::new(String::from("hikchoi"));
    hello(&me);
}

fn main() {
    one();
    two();
    three();
    four();
    five();
    six();
}
