#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?} ", state);
            25
        }
    }
}


fn main() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State Quater from {:?}  ", state),
        _ => count += 1
    }
}
