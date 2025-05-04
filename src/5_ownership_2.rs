fn main() {
    let s = String::from("hello");  // Masuk scope

    /*
    Ketika kita memanggil takes_ownership(s), nilai yang ada di dalam s (yaitu sebuah String) 
    dipindahkan (moved) ke dalam function tersebut. Artinya, setelah pemanggilan fungsi, 
    variabel s di main tidak lagi valid.

    Fungsi takes_ownership menerima parameter bertipe String, yang memiliki alokasi memory di heap. 
    Ketika nilai s dipindahkan ke dalam fungsi, tidak ada salinan yang dibuat; 
    yang terjadi adalah pindah kepemilikan (ownership transfer). 
    Begitu some_string keluar dari scope di dalam fungsi, Rust akan memanggil drop() 
    untuk membersihkan memory heap yang sebelumnya digunakan oleh string tersebut.

    Di sini, s "hilang" setelah dipindahkan, dan jika kamu coba pakai s lagi setelah itu, 
    Rust akan mengeluarkan error karena s sudah tidak valid lagi.
     */
    takes_ownership(s);          
                                   
    /*
    Di sisi lain, tipe seperti i32 adalah tipe yang mengimplementasikan trait Copy. 
    Ketika kita passing x ke fungsi makes_copy(x), Rust tidak memindahkan x seperti yang terjadi pada String. 
    Sebaliknya, Rust hanya membuat salinan dari nilai tersebut dan memindahkannya ke dalam fungsi.

    Jadi, meskipun kita memindahkan x ke dalam fungsi makes_copy, nilai asli x yang ada di dalam main tetap 
    bisa digunakan setelah pemanggilan fungsi karena i32 di-copy, bukan di-move.
     */
    let x = 5; // Masuk scope
    makes_copy(x);                           
    println!("{}", x);             

    /*
    Apa yang Terjadi Saat Selesai Dengan Scope?
    Begitu fungsi selesai, Rust akan mengatur memori untuk tipe yang dipindahkan dan yang disalin dengan cara yang berbeda:
    - Tipe yang dipindahkan (seperti String): Memori yang digunakan oleh data itu akan dibersihkan 
      ketika variabel keluar dari scope, menggunakan fungsi drop().
    - Tipe yang disalin (seperti i32): Tidak ada yang spesial terjadi, 
      karena x tidak memerlukan alokasi khusus yang perlu dibersihkan.
     */

} 

fn takes_ownership(some_string: String) { 
    println!("{some_string}");
} 

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}