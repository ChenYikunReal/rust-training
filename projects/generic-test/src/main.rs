// 不能运行这个函数，这个函数纯属demo
fn max<T>(array: &[T]) -> T {
    let mut max_index:i32 = 0;
    let mut i:i32 = 1;
    while i < array.len() as i32 {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Color<T> {
    Red(T), Green(T), Blue(T)
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
}
