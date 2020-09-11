fn main() {
    // 字符串切片
    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{}={}+{}", s, part1, part2);

    // String => &str
    let s1 = String::from("hello");
    let s2 = &s1[..];

    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
