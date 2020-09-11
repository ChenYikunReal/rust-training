use std::fs;

fn main() {
    let text1 = fs::read_to_string("test.txt").unwrap();
    println!("{}", text1);
    fs::write("test.txt", "LMNOPQ").unwrap();
    let text2 = fs::read_to_string("test.txt").unwrap();
    println!("{}", text2);
}
