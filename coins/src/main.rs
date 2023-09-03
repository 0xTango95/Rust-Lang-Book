#[derive(Debug)]
enum UKCity {
    London,
    Manchester,
    Newcastle,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UKCity),
}

fn main() {
    let penny = Coin::Penny;

    let newcastle = Coin::Quarter(UKCity::Newcastle);
    let result = value_in_cents(newcastle);
    println!("The result was {}", result);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
