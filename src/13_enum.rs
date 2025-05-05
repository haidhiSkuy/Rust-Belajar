enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("🚪 Quit the app!"),
            Message::Move { x, y } => println!("🚀 Moving to position: ({}, {})", x, y),
            Message::Write(text) => println!("📝 Writing message: {}", text),
            Message::ChangeColor(r, g, b) => println!("🎨 Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}


fn main() { 
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 42, y: 24 };
    let m3 = Message::Write(String::from("Hello world!"));
    let m4 = Message::ChangeColor(255, 0, 255);

    m1.call();
    m2.call();
    m3.call();
    m4.call();


}