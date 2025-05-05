fn main(){
    let user1 = User { 
        active : true, 
        username : String::from("haidhi"), 
        email : String::from("haidhi@yahoo.com"), 
        sign_in_count : 1,
    }; 
    println!("{}", user1.username);
    println!("{}", user1.email);

    // Mutable 
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };    
    user2.email = String::from("anotheremail@example.com");

    // Struct Update Syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    }; // Artinya: ambil semua field dari user1 KECUALI yang disebut eksplisit (dalam hal ini email)
    println!("{}", user1.active);
    println!("{}", user1.email);
    // println!("{}", user1.username); Error: tipe non-copy field username udah pindah dari user1 ke user3
    
    let user4 = build_user(String::from("hehe@yahoo.com"), String::from("Sanz")); 
    

}
struct User {
    active: bool, 
    username: String, 
    email: String, 
    sign_in_count: u64,
}

// Field Init Shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // ini maksudnya: username: username
        email,    // dan ini: email: email
        sign_in_count: 1,
    }
}
