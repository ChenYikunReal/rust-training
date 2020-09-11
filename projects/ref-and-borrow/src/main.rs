fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1={}, s2={}", s1, s2);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let s3 = String::from("hello");
    let s4 = &s1;
    let s5 = s3;
    // 因为s4租借的s3已经将所有权移动到s5，所以 s4将无法继续租借使用s3的所有权。如果需要使用s4使用该值，必须重新租借
    // println!("{}", s4);
    // 应该这样
    let s4 = &s5;

    // s4.push_str("test"); // 错误，禁止修改租借的值 => 只读不可写
    // 这样定义就可以改
    let mut s6 = String::from("run");
    // s6 是可变的
    let s7 = &mut s6;
    // s7 是可变的引用
    s7.push_str("test");
    println!("{}", s7);
    // 最后说明：可变引用不可以多重引用；不可变引用可以多重引用
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
