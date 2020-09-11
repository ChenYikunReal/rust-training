fn f1() {
    println!("Hello, world! @f1()");
}

fn main() {
    println!("Hello, world! @main()");
    f1();
    f2(3, 4);
    f3();
    fn five() -> i32 {
        5
    }
    println!("five()={}", five());
    println!("3+2={} @f4()", f4(3, 2));
}

fn f2(x: i32, y: i32) {
    println!("x={}, y={} @f2()", x, y);
}

fn f3() {
    let x = 5;
    let y = {
        let x = 3;
        // 如果x+1后有分号的话就变成了一个语句，这里就语法错误了。
        x + 1
    };
    println!("x={}, y={} @f3()", x, y);
}

fn f4(a: i32, b: i32) -> i32 {
    return a + b;
}
