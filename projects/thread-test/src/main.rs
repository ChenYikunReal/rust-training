use std::thread;
use std::time::Duration;

fn main() {
    // 使用闭包传递会比较好
    // 闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。闭包相当于Lambda表达式
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
