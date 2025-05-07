#[derive(Debug)] // agar bisa dicetak dengan {:?}
enum UsState {
    Alabama,
    Alaska,
}

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
            println!("State quarter from {state:?}!");
            25
        },
    }
}

/*
Fungsi plus_one ini menerima parameter bertipe Option<i32>. Artinya, bisa jadi:
 - Ada nilai i32, dibungkus dalam Some.
 - Tidak ada nilai sama sekali (None).
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    /*
    match akan membandingkan nilai x dengan setiap pola (pattern) yang ada, 
    dan mengeksekusi blok kode yang sesuai.
    Jika x adalah None, maka akan langsung mengembalikan None.
    Jika x adalah Some(i), maka:
      - i akan menjadi nilai di dalam Some, misalnya Some(5) → i = 5
      - Fungsi akan mengembalikan Some(i + 1), yaitu Some(6)
     */
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let my_coin: Coin = Coin::Penny; 
    let my_coin_cent = value_in_cents(my_coin); 
    println!("My coin value is {}", my_coin_cent);

    let my_coin2: Coin = Coin::Quarter(UsState::Alabama); 
    let my_coin2_cent = value_in_cents(my_coin2); 
    println!("My coin2 value is {}", my_coin2_cent);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // Catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => println!("{}", dice_roll),
        7 => println!("{}", dice_roll),
        other => println!("{}", dice_roll),
    }
}