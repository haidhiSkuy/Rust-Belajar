use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file, 
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("gagal bikin file: {e:?}")
            })
        } else { 
            panic!("Gagal buka file: {error:?}")
        }
    });

}
