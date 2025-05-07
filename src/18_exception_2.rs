use std::fs::File;

fn main() { 
    /*
    Metode unwrap() adalah cara cepat untuk menangani Result yang kita harapkan akan selalu sukses. 
    Jika hasilnya adalah Ok(T), maka unwrap() akan mengembalikan nilai yang ada di dalam Ok. Namun, 
    jika hasilnya adalah Err(E), maka unwrap() akan memicu panic! dengan pesan error bawaan.
     */
    let greeting_file = File::open("hello.txt").unwrap();  

    /*
    Metode expect() mirip dengan unwrap(), namun dengan perbedaan utama: kita bisa menentukan pesan error 
    yang akan ditampilkan saat panic terjadi. Dengan expect(), kita dapat memberikan konteks yang lebih 
    jelas tentang alasan kenapa operasi ini seharusnya selalu berhasil.
     */
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}