fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);
    println!("{}", match v1.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });
    for i in &v1 {
        print!("{}, ", i);
    }
    println!();
    for j in &mut v1 {
        *j += 50;
    }
    for i in &v1 {
        print!("{}, ", i);
    }
}
