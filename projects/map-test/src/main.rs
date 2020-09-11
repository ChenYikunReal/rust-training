use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    for p in map.iter() {
        println!("{:?}", p);
    }
    if let Some(x) = map.get_mut(&"color") {
        *x = "yellow";
    }
    for p in map.iter() {
        println!("{:?}", p);
    }
}
