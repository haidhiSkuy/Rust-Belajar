fn main(){
    let s = String::from("hello world");
    let slice = &s[0..2]; // “he”
    println!("{}", slice);

    let s_first_word = first_word(&s); 
    println!("{}", s_first_word);

    let s_second_word = second_word(&s); 
    println!("{}", s_second_word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]

} 

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let len = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..len];
        }
    }
    &s[..]
}