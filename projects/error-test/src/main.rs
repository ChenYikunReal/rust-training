use std::io;
use std::io::Read;
use std::fs::File;

fn panic_test() {
    panic!("error occured");
}

fn ioe1() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }
}

fn ioe2() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
}

fn error_pass(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) } else { Err(false) }
}

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // panic_test();

    // ioe1();

    // let r = error_pass(10000);
    // if let Ok(v) = r {
    //     println!("Ok: f(-1) = {}", v);
    // } else {
    //     println!("Err");
    // }

    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
