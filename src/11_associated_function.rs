/*
Associated Function
fungsi-fungsi yang nempel sama struct tapi nggak butuh objek-nya buat jalan. 
Jadi mereka itu bukan method, tapi tetep bisa numpang tinggal di dalam impl block
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let kotak = Rectangle::square(5);
    println!("{:?}", kotak);
}
