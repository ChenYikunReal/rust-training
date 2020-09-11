fn main() {
    // 定义变量
    let a = 12;
    // 使用两遍占位符
    println!("a is {}, a again is {}", a, a);
    // 简化占位符为1次
    println!("a is {0}, a again is {0}", a);
}
