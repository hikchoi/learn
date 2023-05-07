use core::slice;

fn _one() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn _two() {
    // unsafe functions
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn _three() {
    // creating safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let (a, b) = my_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

}

fn _four() {
    // calling external code
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }
}

fn _five() {
    // accessing or modifying mutable static variable
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    _six();
}
