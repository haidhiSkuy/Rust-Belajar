fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // ternary
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Looping
    let mut counter = 0; 
    loop { 
        println!("Hello {}", counter);
        if counter == 10 { 
            break
        }
        counter += 1; 
    }

   // Loop with a value
    let mut counter = 0; 
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}"); 

    // Nested Loop
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; 
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Loop Array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

}