// Multibound Trait

struct Human;
struct Alien; 

pub trait Greet {
    fn greet(&self) -> String;    
}
pub trait Attack {
    fn attack(&self) -> String;
}

impl Greet for Human {
    fn greet(&self) -> String {
        String::from("Hello Human")
    }
}
impl Attack for Human {
    fn attack(&self) -> String {
        "Human attacking".into()
    }
}

impl Greet for Alien {
    fn greet(&self) -> String {
        String::from("Hello Alien")
    }
}
impl Attack for Alien {
    fn attack(&self) -> String {
        "Alien attacking".into()
    }
}

fn say_hello<T: Greet + Attack>(entity: T){
    println!("{}", entity.greet());
    println!("{}", entity.attack())
}

fn main() {
    let h = Human;
    let a = Alien; 
    
    say_hello(h);
    say_hello(a);

}
