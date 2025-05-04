fn main() {
    /*
    Fungsi gives_ownership mengembalikan sebuah nilai bertipe String. 
    Ketika nilai itu dipassing kembali ke main(), nilai tersebut dipindahkan ke dalam variabel s1. 
    Jadi, ownership dari nilai yang sebelumnya ada di dalam some_string sudah berpindah ke s1 
    dan tidak bisa lagi dipakai di tempat lain.
     */
    let s1 = gives_ownership();         
    println!("{}",s1);

    /*
    Fungsi takes_and_gives_back mengambil parameter bertipe String dan mengembalikan nilai yang sama ke 
    pemanggilnya. Ketika s2 dipassing ke dalam fungsi ini, ownership berpindah ke dalam fungsi tersebut, 
    dan begitu fungsi selesai, nilai yang dikembalikan dipindahkan lagi ke variabel s3.
    */
    let s2 = String::from("hello");   
    let s3 = takes_and_gives_back(s2);  
    println!("{}",s3);

    /*
    Pada akhir main, variabel s1, s2, dan s3 akan diproses sesuai dengan ownership mereka masing-masing: 
    s1 dan s3 akan dibersihkan (dihapus dari memory) begitu keluar dari scope-nya, sedangkan s2 
    sudah tidak bisa digunakan lagi setelah dipindahkan ke dalam fungsi takes_and_gives_back.
     */


    // ######################### MULTIPLE RETURNS ########################
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    // Jika tidak ingin transfer ownership
    let s1 = String::from("Wkwk"); 
    let len = calculate_length2(&s1);
    println!("The length of '{s1}' is {len}.");


} 

fn gives_ownership() -> String {                                             
    let some_string = String::from("yours"); 
    some_string                             
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string  
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length2(s : &String) -> usize { 
    let length = s.len(); 
    length
}