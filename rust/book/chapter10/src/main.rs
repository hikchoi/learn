// this needs a generic lifetime parameter
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// like this
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn one() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn two() {
    // lifetime: until the end of the outer scope
    let string1 = String::from("long string is long");
    {
        // lifetime: until the end of the inner scope
        let string2 = String::from("xyz");
        // lifetime: until the end of the inner scope
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn three() {
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     // string2 need to be valid until the end of the outer scope for result to be valid for println!
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

}

// lifetime annotations in struct definitions
fn four() {
    // 'a annotation means an instance of ImportantExcerpt
    // cannot outlive the referenced string slice 'part'
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// lifetime elision
fn five() {
    // the reason this can compile without any lifetime annotation is
    // because the rust analyzer can infer some specific lifetime patterns
    // this is called lifetime elision rules.
    // if there are still ambiguities, the rust analyzer will give you an error
    // which you can use to properly annotate the lifetime
    // first rule: each input gets assigned a different lifetime annotation
    // second rule: if there are only one input, the output gets the same lifetime annotation
    // third rule: if there are multiple input lifetime parameters, but one of them is &self or &mut self,
    // the lifetime of self is assigned to all output lifetime parameters
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}

// lifetime annotations in method definitions
fn six() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    // rule 1 lets us omit the lifetime annotation on &self
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // with rule 1, &self and announcement gets their own lifetime annotation
    // because one of the parameters are &self, the return tyupe gets the lifetime of &self
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("attention please: {}", announcement);
            self.part
        }
    }
}

// static lifetime
fn seven() {
    // this can live until the end of the program.
    let s: &'static str = "I have a static lifetime.";
}

// generic type parameters, trait bounds, and lifetimes together
fn eight() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn main() {
    one();
    two();
}
