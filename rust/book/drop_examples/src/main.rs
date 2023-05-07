struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn one() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created!");
}

// dropping a value early
fn two() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

fn main() {
    one();
    two();
}
