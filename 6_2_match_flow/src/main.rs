fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("Value is {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("None: {:?}", none);
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
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Licky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
