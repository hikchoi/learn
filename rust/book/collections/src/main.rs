use std::vec;


fn one() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn two() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn three() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // borrow checker won't allow this
    // v.push(6);

    println!("The first element is: {}", first);
}

fn four() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}", n_plus_one);
    }

    let mut v2 = vec![100, 32, 57];
    for n_ref in &mut v2 {
        *n_ref += 50;
    }
}

fn five() {
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();
}

fn six() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

fn seven() {
    {
        let v = vec![1, 2, 3, 4];
    }
}

fn main() {
    one();
    two();
    three();
    four();
    five();
    six();
    seven();
}
