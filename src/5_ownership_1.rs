fn main(){
    /*
    String Literal
    - disimpen di memory read-only section (bukan heap/stack)
    - ukuran diketahui pas compile
    - constant, no change, no touchy
     */ 
    let s = "hello"; 
    println!("{}", s);

    /*
    String
    - dari standard library 
    - langsung masuk heap
    - ukuran fleksibel
    - mutable
     */
    let mut s = String::from("Hello"); 
    s.push_str(", World");

    /*
    RAII (Resource Acquisition Is Initialization)
    Setiap object yang "punya" resource, bakal otomatis ngelepasin itu pas dia mati
     */
    {
        let mut s2 = String::from("Hello");
        s2.push_str(", World 2");
    } // ⛔️ s ilang dari scope ➡️ Rust langsung manggil `drop()` ➡️ memori dibalikin


    // ############################## OWNERSHIP #############################
    /*
    Ownership
    Karena x adalah tipe data primitif (seperti i32), itu langsung disalin ke dalam variabel y. 
    Jadi, x dan y akan menyimpan nilai yang sama (5), dan karena tipe data primitif ini memiliki ukuran tetap 
    dan disimpan di stack, kita tidak perlu khawatir tentang manajemen memori yang rumit.
    Namun, hal yang sama tidak berlaku untuk tipe data yang lebih kompleks, 
    seperti String, yang dikelola di heap.
     */
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    /*
    Ownership string
    Terlihat mirip dengan contoh di atas, tetapi hasilnya berbeda. 
    Ketika kita menulis let s2 = s1;, Rust tidak membuat salinan dari data string di heap. 
    Sebagai gantinya, Rust memindahkan (move) ownership dari s1 ke s2. Ini berarti, 
    s2 sekarang memiliki data string itu, dan s1 tidak lagi valid setelahnya. 
    Kita bisa bilang bahwa s1 “terhapus” atau "terputus" dari data string di heap.
    Jadi, yang terjadi di memori adalah sebagai berikut:
    - s1 pada stack menyimpan pointer ke data yang ada di heap, serta informasi tentang panjang 
      dan kapasitas string.
    - Ketika s1 dipindahkan ke s2, yang sebenarnya terjadi adalah s2 sekarang memegang pointer, panjang, 
      dan kapasitas yang sama, mengarah ke data string yang sama di heap.
     */
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{} {}", s1, s2); Error
    println!("{}", s2);

    // ######################################## CLONE & COPY #########################################
    /*
    Clone
    Metode clone digunakan ketika kita ingin membuat salinan mendalam dari sebuah objek.
    Artinya, kita membuat salinan yang terpisah dari semua data yang disimpan pada heap, 
    bukan hanya menyalin pointer atau referensi.

    Di sini, s2 akan memiliki salinan lengkap dari string yang ada di heap, termasuk data itu sendiri, bukan hanya pointer-nya. Jadi, jika kamu mencetak s1 dan s2, keduanya akan berfungsi secara independen, dan keduanya valid.

    Kenapa clone itu "mahal"?
    - Karena clone melakukan salinan mendalam (deep copy), 
      yang berarti kalau data yang disalin ada di heap (seperti String), 
      itu membutuhkan waktu dan sumber daya lebih banyak untuk menduplikasi seluruh isi dari heap tersebut.
    - Biasanya, kamu akan ingin menghindari penggunaan clone kalau bisa, karena bisa memperlambat program.
     */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    /*
    Copy
    Sekarang, untuk tipe data yang lebih sederhana seperti integer atau tipe 
    yang memiliki ukuran tetap (fixed size) yang sudah diketahui saat kompilasi, 
    Rust tidak membutuhkan salinan mendalam seperti di atas. 
    Untuk tipe seperti ini, Rust cukup melakukan salinan dangkal atau shallow copy secara otomatis. 
    Ini disebut Copy trait.

    Di sini, x dan y sama-sama memiliki nilai 5. Tidak ada perubahan apapun di memory heap, 
    dan kedua variabel ini bisa dipakai tanpa masalah. Rust melakukan salinan dari nilai tersebut, 
    dan itu sangat cepat karena data tipe tersebut (seperti integer) ada di stack, bukan heap.

    Kenapa Copy bisa lebih efisien?
    Tipe data yang masuk dalam kategori Copy seperti integer, boolean, dan floating-point, 
    memiliki ukuran tetap dan sederhana, jadi mereka bisa disalin secara cepat dan efisien 
    tanpa memerlukan operasi salinan mendalam.
    */
    let x = 5; 
    let y = x;
    println!("{} {}", x, y);
    
    /*
    Kenapa Rust Memisahkan Ini?
    Rust memisahkan dua konsep ini untuk menghindari overhead yang tidak perlu:
    - Tipe yang lebih sederhana (seperti integer dan boolean) disalin dengan cara yang sangat 
      efisien di stack tanpa perlu clone.

    - Tipe yang lebih kompleks (seperti String dan Vec) disalin dengan menggunakan clone, 
      karena mereka melibatkan alokasi memory di heap dan membutuhkan salinan 
      mendalam agar data tetap aman dan terisolasi antar variabel.
     */

}