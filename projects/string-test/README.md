# Rust字符串

```rust
fn main() {
    let string = String::new();      //新建字符串

    let one = 1.to_string();         // 整数到字符串
    let float = 1.3.to_string();     // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!');       // 追加字符

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    // 格式化
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 长度
    let s = "hello";
    let len = s.len();  //长度为5
    let s = "你好";
    let len = s.len();  //长度为6

    let s = "hello你好";
    let len = s.chars().count(); //一共7个字符

    // 遍历字符串
    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }

    // 取出单个字符
    let s = String::from("EN中文");
    let a = s.chars().nth(2);
    println!("{:?}", a);

    // 截取字符串 小心不要截断UTF-8字符
    let s = String::from("EN中文");
    let sub = &s[0..2];
    println!("{}", sub);
}
```
