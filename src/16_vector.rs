enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() { 
    let mut v1: Vec<i32> = Vec::new(); // empty vector
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);


    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5, 6]; 
    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("{}",i);
    } 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}