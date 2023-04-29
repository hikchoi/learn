fn main() {
    one();
    two();
}

// write a function that takes a string of words
// separated by spaces and returns the first word it finds in that string.
fn one() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();

    println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len() 
}

// string slices
fn two() {
    // let mut s = String::from("hello");

    // let hello: &str = &s[0..5];
    // println!("{hello}");

    // s.push_str(" world");
    let mut s = String::from("hello world");

    let word = first_word2(&s);

    println!("the first word is: {}", word);
    s.clear();

    let my_string = String::from("hello world");

    let _word = first_word3(&my_string[0..6]);
    let _word = first_word3(&my_string[..]);

    let _word = first_word3(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word3(&my_string_literal[0..6]);
    let _word = first_word3(&my_string_literal[..]);
    let _word = first_word3(my_string_literal);
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}





