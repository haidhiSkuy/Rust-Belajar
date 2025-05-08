use std::fmt::{self, Display};

// Definisikan struct NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Definisikan struct Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Definisikan trait Summary
pub trait Summary {
    fn summarize_author(&self) -> String; // Wajib diimplementasikan

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // Default implementation
    }
}

// Implementasikan trait Summary untuk NewsArticle
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// Implementasikan trait Summary untuk Tweet
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Implementasikan trait Display untuk NewsArticle
impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.headline, self.content)
    }
}

// Implementasikan trait Display untuk Tweet
impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

// Fungsi notify menggunakan trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Fungsi notify dengan multiple trait bounds (Summary dan Display)
pub fn notify_with_display<T: Summary + Display>(item: &T) {
    println!("Item: {}", item);        // Pakai Display
    println!("Summary: {}", item.summarize());
}

fn main() {
    // Membuat instance NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins are the best in NHL!"),
    };

    // Memanggil notify untuk NewsArticle
    notify(&article); // Output: Breaking news! (Read more from @Iceburgh...)

    // Membuat instance Tweet
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("people will be able to use this for years to come!"),
        reply: false,
        retweet: false,
    };

    // Memanggil notify untuk Tweet
    notify(&tweet); // Output: Breaking news! (Read more from @horse_ebooks...)

    // Memanggil notify_with_display untuk Tweet
    notify_with_display(&tweet); // Output: Item: horse_ebooks: people will be able to use this for years to come!
                                //          Summary: (Read more from @horse_ebooks...)
}
