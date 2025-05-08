// Trait as Parameter

struct Human;
struct Robot; 

pub trait Greet {
    fn greet(&self) -> String; 
}

impl Greet for Human {
    fn greet(&self) -> String {
        String::from("Hello Human")
    }
}

impl Greet for Robot {
    fn greet(&self) -> String {
        String::from("Hello Robot")
    }
}

fn say_hello(entity: &impl Greet) {
    println!("{}", entity.greet());
}

fn main() { 
    let h = Human;
    let r = Robot; 
    
    say_hello(&h);
    say_hello(&r);
}