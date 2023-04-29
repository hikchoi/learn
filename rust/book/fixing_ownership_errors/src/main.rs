use std::{rc::Rc, vec};
fn main() {
    // let str = return_a_string();    
    let str1 = return_a_string1();
    println!("{str1}");
    let str2 = return_a_string2();
    println!("{str2}");
    let str3 = return_a_string3();
    println!("{str3}");
    let mut output = String::from("");
    return_a_string4(&mut output);
    println!("{output}");

    not_enough_permissions();
}

// wrong code
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// fixes
// 1. how long should my string live?
// 2. who should be in charge of deallocating it?

// strategy 1: move ownership of the string
fn return_a_string1() -> String {
    let s = String::from("Hello world");
    s
}

// strategy 2: return a string literal
fn return_a_string2() ->  &'static str {
    "Hello world"
}

// strategy 3: defer lifetime-checking to runtime (use gc)
fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

// strategy 4: provide a slot to put the string using a mutable reference
fn return_a_string4(output: &mut String) {
    output.replace_range(.., "Hello world");
}

// not enough permissions
fn not_enough_permissions() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let full = stringify_name_with_title3(&name);
    println!("{first}");
    println!("{full}");
}

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     // rejected by the borrow checker.
//     // needs W permission but name is an immutable reference
//     name.push(String::from("Esq.")); 
//     let full = name.join(" ");
//     full
// }

// strategy 1: make arg name a mutable reference
// not good since this is not the sort of function that is expected to mutate the arg
// fn stringify_name_with_title1(name: &mut Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// strategy 2: clone name
// fn stringify_name_with_title2(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));
//     let full = name_clone.join(" ");
//     full
// }

// strategy 3: avoid cloning
fn stringify_name_with_title3(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// alaiasing and mutating a data structure


// fn add_big_strings(dst: &mut vec<string>, src: &[string]) {
//     let largest: &string = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             // unsafe because dst.push can deallocate contents of dst
//             // which can invalidate the reference `largest`
//             dst.push(s.clone());
//         }
//     }
// }
// shorten the lifetime of borrws on dst to not overlap with a mutation to dst.

// strategy 1: shorten the lifetime of `largest` to not overlap with dst.push
// this may cause a performance hit.
fn _add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// strategy 2: perform all the length comparison first, then mutat dst afterwards
// also causes performance hit for allocating to_add vector
fn _add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> =
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}

// strategy 3: copy length. most idiomatic and performant
fn _add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// copying vs moving out of a collection
fn _copying_vs_moving() {
    // copies a number out of a vector (safe)
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;
    println!("{n}");

    // doesn't work with strings
    // a double free is occuring on the heap.
    // after s is dropped, "Hello world" is deallocated.
    // then v is dropped, which is an undefined behavior (because it's trying to deallocate an already deallocated heap)
    // this did not happen with i32 because i32 does not own heap data.
    // let v2: Vec<String> = vec![String::from("Hello world")];
    // let s_ref: &String = &v2[0];
    // let s: String = *s_ref;
    // println!("{s}");

    // if a value does not own heap data, it can be copied without a move.

    // strategy 1: avoid taking ownership of the string and use immutable ref
    let v3: Vec<String> = vec![String::from("Hello world")];
    let s_ref3: &String = &v3[0];
    println!("{s_ref3}");

    // strategy 2: clone the data
    let v4: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v4[0].clone();
    s.push('!');
    println!("{s}");

    // strategy 3: use Vec::remove
    let mut v5: Vec<String> = vec![String::from("Hello world")];
    let mut s5: String = v5.remove(0);
    s5.push('!');
    println!("{s5}");
    assert!(v.len() == 0);
}

fn _mutating_different_tuple_fields() {
    // this is safe. name.1 still has W permission so name.1.push_str can happen
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    // just refactoring a bit make this fail the borrow checker.
    // the compiler doesn't look at what is happening in get_first
    // so name as a whole does not have W permission.
    // fn get_first(name: &(String, String)) -> &String {
    //     &name.0
    // }

    // let first = get_first(&name);
    // name.1.push_str(", Esq.");
    // println!("{first} {}", name.1);

    // then how should we deal with this?
    // we should either inline &name.0, or use cells (covered later)
}

// mutating different array elements
fn _mutating_different_array_elements() {
    // let mut a = [0, 1, 2, 3];

    // // borrow checker does not contain different paths for all indices.
    // // it can't know which is being borrowed. so it assumes everything can be.
    // let x = &mut a[0];

    // // borrow checker will reject this because a's R permission is held by x
    // // this code is safe, but the borrow checker cannot determine that.
    // let y = &a[1];
    // *x += *y;

    // println!("{a:?}");

    // we can do something like this instead:
    let mut a = [0, 1, 2, 3];
    // split_first_mut is an unsafe block.
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    // above is equivalent to this:
    // should normally not do this.
    // this is sometimes necessary, but always prefer std lib functions that do it for you.
    let mut a2 = [0, 1, 2, 3];
    let x = &mut a2[0] as *mut i32;
    let y = &a2[1] as *const i32;
    unsafe { *x += *y }
}