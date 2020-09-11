use std::thread;
use std::sync::mpsc;

fn main() {
    // Rust中一个实现消息传递并发的主要工具是channel，channel由两部分组成，transmitter和receiver
    let (tx, rx) = mpsc::channel();

    // 使用move禁止子线程使用当前函数线程的资源
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
