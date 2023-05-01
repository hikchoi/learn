use core::num::flt2dec::strategy;

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // less typing / indentation / boilerplate
    // lose exhaustive checking though.
    // this is a syntax sugar for match that runs on one pattern and ignore the rest
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    // we can also add esle block to if let
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
}
