// Tuple Struct

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let green = Color(0, 255, 0);
    let origin = Point(0, 0, 0);

    println!("{}", green.1); 
    let Point(x, y, z) = origin;
    println!("{} {} {}", x, y, z); 
}