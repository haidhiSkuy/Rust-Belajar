fn main() {
    let mut s = String::from("hello");
    
    /*
    Aturan 1:
    Boleh memakai banya immutable reference
     */
    let r1 = &s; 
    let r2 = &s; 
    println!("{}, {}", r1, r2);

    /*
    Aturan 2:
    Tidak boleh ada immutable (&) dan mutable (&mut) dalam satu waktu
     */
    // let r1 = &s;
    // let r2 = &mut s; 
    // println!("{}, {}", r1, r2); error 

    /*
    Aturan 3:
    Tidak boleh pakai banyak mutable
     */
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); // error

    /*
    Contoh yang benar dan aman
     */
    // Immutable references, aman
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // Aman, karena cuma baca

    // Setelah r1 dan r2 gak dipakai lagi, kita bisa pake &mut
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);

    /*
    Contoh pakai scope
     */
    {
        let r1 = &mut s;
        r1.push_str(", world");
    } // r1 selesai di sini

    let r2 = &mut s;
    r2.push_str("!!!");

    println!("{}", s);


}
