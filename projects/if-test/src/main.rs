fn main() {
    let a = 12;
    let b;
    // 普通
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b = {}", b);

    // 三目
    let c = if a > 0 {1} else {-1};
    println!("c = {}", c);
}
