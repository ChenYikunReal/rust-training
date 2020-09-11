fn main() {
    let s = String::from("hello");
    // s被声明有效
    takes_ownership(s);
    // s的值被当作参数传入函数
    // 所以可以当作s已经被移动，从这里开始已经无效
    let x = 5;
    // x被声明有效
    makes_copy(x);
    // x的值被当作参数传入函数
    // 但x是基本类型，依然有效
    // 在这里依然可以使用x却不能使用s
    let s1 = gives_ownership();
    // gives_ownership移动它的返回值到s1
    let s2 = String::from("hello");
    // s2被声明有效
    let s3 = takes_and_gives_back(s2);
    // s2被当作参数移动, s3获得返回值所有权
} // 函数结束, x无效, 然后是s. 但s已被移动, 所以不用被释放。此时，s3无效被释放, s2被移动, s1无效被释放。

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效
    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效
    a_string  // a_string 被当作返回值移出函数
}
