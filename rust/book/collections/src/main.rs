use std::vec;

// vectors
fn _one() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn _two() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn _three() {
    let mut _v = vec![1, 2, 3, 4, 5];
    let first = &_v[0];

    // borrow checker won't allow this
    // v.push(6);

    println!("The first element is: {}", first);
}

fn _four() {
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

fn _five() {
    let mut _v: Vec<i32> = vec![1, 2];
    let mut iter = _v.iter();
    let _n1: &i32 = iter.next().unwrap();
    let _n2: &i32 = iter.next().unwrap();
    let _end: Option<&i32> = iter.next();
}

fn _six() {
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

fn _seven() {
    {
        let _v = vec![1, 2, 3, 4];
    }
}

// string
fn _eight() {
    let data = "initial contents";

    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    // String::from and to_string do the same things

    // updating a string
    let mut _s = String::from("foo");
    _s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut _s3 = String::from("lo");
    _s3.push('l');

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let _s6 = s4 + &s5;

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let _s10 = s7 + "-" + &s8 + "-" + &s9;
    // let s11 = format!("{}-{}-{}", s7, s8, s9);
}

// hash maps
fn _nine() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn _ten() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn _eleven() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn _twelve() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    _twelve();
}
