use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("🎮 Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess_number: i32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number, not gibberish!");
                continue;
            }
        };

        let guess = match validate_guess(guess_number) {
            Ok(g) => g,
            Err(msg) => {
                println!("🚫 {}", msg);
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("📉 Too low!"),
            Ordering::Greater => println!("📈 Too high!"),
            Ordering::Equal => {
                println!("🎉 You got it! The number was {}.", secret_number);
                break;
            }
        }
    }
}

fn validate_guess(num: i32) -> Result<Guess, String> {
    if num < 1 || num > 100 {
        Err(String::from("Number must be between 1 and 100."))
    } else {
        Ok(Guess::new(num))
    }
}
