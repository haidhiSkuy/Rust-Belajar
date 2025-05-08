fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let str1 = "tes";
    let str2 = "testos"; 

    let longest_word = longest(&str1, &str2); 
    println!("Longest is {}", longest_word);
    
}   