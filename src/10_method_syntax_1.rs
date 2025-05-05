#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 { 
        let prm = 2 * (self.width + self.height);
        prm
    }

    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20, 
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40, 
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The perimeter of the rectangle is {}",
        rect1.perimeter()
    );

    let rect1_can_hold_rect2 = rect1.can_hold(&rect2); 
    println!("rect1 can hold rect2 : {}", rect1_can_hold_rect2);

    let rect1_can_hold_rect3 = rect1.can_hold(&rect3); 
    println!("rect1 can hold rect3 : {}", rect1_can_hold_rect3);
  
}