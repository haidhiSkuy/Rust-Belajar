fn main() {
    let x: i32 = -10; // int
    let x: u32 = 10; // unsigned int 

    let x: f32 = 3.14; // float 

    let is_rain: bool = true; // boolean 
    match is_rain {
        true => println!("rain"),
        false => println!("sunny")
    }

    // tupple
    let tup: (i32, f32, String) = (100, 3.14, "wkwk".to_string()); 
    println!("{} {} {}", tup.0, tup.1, tup.2); 

    // Array 
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{} {} {} {}", months[0], months[1], months[2], months[3]);
}   
