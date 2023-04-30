enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { 
            println!("Lucky Penny!");
            1 
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matches are exhaustive.
    // if we leave out any line under, it won't compile
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn main() {
    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        5 => move_player(5),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("Moving player {num_spaces} spaces");
    }
    fn reroll() {}

    // match ownership
    let opt: Option<String> = Some(String::from("Hello World"));

    match opt {
        // we are not borrowing from the non-copyable string data in some
        // if we do, this will not compile
        // e.g. Some(s) => println!("Some: {}", s),
        // in this case, the ownership has moved, so we can't do the print at the end.
        // if we don't want that to happen, we can match on the reference
        Some(_) => println!("Some!"),
        None => println!("None"),
    }

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    }

    println!("{:?}", opt);
}
