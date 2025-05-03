fn main() {
    let a : f64 = 10.0; 
    let b : f64 = 20.0; 

    println!("{}", segitiga(a, b));

    // Expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}   


fn segitiga(a : f64, b : f64) -> f64 { 
    a * b / 2.0
}