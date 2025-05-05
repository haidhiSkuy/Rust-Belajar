// Example struct app 

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );
}

// Using tuple
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// using struct
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
struct Rectangle {
    width: u32,
    height: u32,
}