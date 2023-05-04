fn one() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    let only_borrows = || println!("From closure: {:?}", list);
    
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn two() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);

    // this won't be allowed until the closure is called and releases the borrow
    // println!("Before calling closure: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn three() {
    use std::thread;

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn main() {
    one();
    two();
    three();
}
