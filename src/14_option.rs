fn main() { 
    let some_number = Some(5);
    match some_number { 
        Some(val) => println!("angka = {}", val), 
        None => println!("No val"),
    } 

    let num1 = Some(1); 
    let num2 = Some(2);
    let num3 = 3;
    let result = match (num1, num2) { 
        (Some(x), Some(y)) => Some(x + y + num3), 
        _ => None,
    };
    println!("{}", result.unwrap());
}