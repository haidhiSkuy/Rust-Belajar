struct Point<T> {
    x: T, 
    y: T
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T> {
    x: T, 
    y: T
}
impl Point2<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let point = Point {x: 5, y: 10};
    println!("x: {}", point.x());

    let point2 = Point2 {x: 5.0, y: 10.0};
    println!("Distance: {}", point2.distance_from_origin());
}