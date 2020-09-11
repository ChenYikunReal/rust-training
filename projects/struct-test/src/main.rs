struct Student1 {
    id: i32,
    name: String,
}

fn main() {
    let sam = Student1 {
        id: i32:: from(123),
        name: String:: from("Sam")
    };

    let id = 234;

    let amy = Student1 {
        id, //等同于id: id
        name: String:: from("Amy"),
    };

    // 元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
