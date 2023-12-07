use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("test_file/test.sql").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file.");

    println!("{}", contents);
}
