fn main() {
    one();
    two();
    three();
    four();
    five();
    six();
}

fn one() {
   let m1 = String::from("Hello");
   let m2 = String::from("world");

   let (m1_again, m2_again) = greet(m1, m2);
   let s = format!("{} {}!", m1_again, m2_again);
   println!("{s}");
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

// references are non-owning pointers

fn two() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet2(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{s}");
}

fn greet2(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

// dereferencing a pointer accesses its data

fn three() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);
    println!("x_abs1: {}, x_abs2: {}", x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);
    println!("r_abs1: {}, r_abs2: {}", r_abs1, r_abs2);
    
    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);
    println!("s_len1: {}, s_len2: {}", s_len1, s_len2);
}

// rust avoids simultaneous aliasing and mutation

fn four() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    println!("Third element is {}", *num);
    vec.push(4);
}

// mutable references provide unique and non-owning access to data

fn five() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;

    println!("Third element is {}", *num);

    println!("Vector is new {:?}", vec);

    let mut text = vec!['W', 'o', 'w'];
    println!("{:?}", text);
    ascii_capitalize(&mut text);
    println!("{:?}", text);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn six() {
    let s = String::from("Hello world");
    let strings = vec![s];
    let s_ref = first(&strings);
    println!("{}", s_ref);
}

fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    return s_ref;
}











