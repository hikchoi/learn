use std::{fs::{File, self}, io::{ErrorKind, Read, self}};

fn one() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn two() {
    // alternative to using matches
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn three() {
    // panic on error -> unwrap and expect
    let _greeting_file1 = File::open("hello.txt").unwrap();
    let _greeting_file2 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file2() -> Result<String, io::Error> {
    // ? can be used as a shorthand for matching
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file3() -> Result<String, io::Error> {
    // even shorter
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn _read_username_from_file4() -> Result<String, io::Error> {
    // even even shorter
    fs::read_to_string("hello.txt")
}

fn main() {
    one();
    two();
    three();
}
