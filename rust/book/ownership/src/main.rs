fn main() {
    one();
    two();
    three();
}

// safety is the absence of undefined behavior
fn one() {
    let x = true;
    read(x);
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

// collections use boxes

fn two() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// cloning avoids moves

fn three() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}