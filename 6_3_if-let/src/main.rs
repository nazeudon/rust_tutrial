fn main() {
    let some_u8_vlaue = Some(0u8);
    if let Some(3) = some_u8_vlaue {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quater(state) = coin {
        println!("State quater from {:?}", state);
    } else {
        count += 1;
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
