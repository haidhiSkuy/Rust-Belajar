#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Quarter(UsState::Alaska);
    let coin3 = Coin::Dime;

    println!("{:?}", describe_state_quarter(coin1));
    println!("{:?}", describe_state_quarter(coin2));
    println!("{:?}", describe_state_quarter(coin3));
}
