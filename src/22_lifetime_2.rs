// Filename: src/main.rs

// 1️⃣ Struct dengan lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 2️⃣ Method tanpa return reference — elision rule #1
    fn level(&self) -> i32 {
        3
    }

    // 3️⃣ Method return reference dari self — elision rule #3
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// 4️⃣ Function yang pakai lifetime elision rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 5️⃣ String literal dengan 'static lifetime
fn static_example() -> &'static str {
    "This lives forever in the binary 💀"
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    // Bikin instance struct yang nyimpen reference
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Call method tanpa return reference
    println!("Level: {}", i.level());

    // Call method dengan return reference (dari field)
    let part = i.announce_and_return_part("🚨 Ini penting bro!");
    println!("Excerpt: {part}");

    // Pakai function first_word
    let word = first_word("Hello world!");
    println!("First word: {word}");

    // Contoh string literal yang punya 'static lifetime
    let eternal = static_example();
    println!("Static string: {eternal}");
}


/*
Tl;dr
- Function: Butuh lifetime ketika mengembalikan referensi yang terkait dengan parameter fungsi, 
  atau ketika kita ingin mencegah referensi yang outlived.
- Struct: Butuh lifetime saat menyimpan referensi, karena struct bisa "outlive" data 
  yang disimpan di dalamnya, jadi kita harus memberi tahu berapa lama data itu berlaku.
- Impl/Method: Butuh lifetime saat method-nya melibatkan referensi dalam parameter atau return type, 
  terutama jika method-nya mengakses data yang ada di dalam struct atau data eksternal.
*/