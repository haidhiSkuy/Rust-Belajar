fn main(){

    let s = no_dangle();
    println!("{}", s);

} 

fn dangle() -> &String {
    let s = String::from("hello");
    &s // ❌ pengen return reference ke sesuatu yang bakal HILANG, karena sudah keluar dari scope
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // kasih whole String-nya, bukan referensinya
}
