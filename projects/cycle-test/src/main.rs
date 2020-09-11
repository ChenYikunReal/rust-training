fn main() {
    // while
    let mut num = 1;
    while num != 4 {
        println!("{}", num);
        num += 1;
    }
    println!("EXIT");


    // for
    let list = [10, 20, 30, 40, 50];
    // for迭代器访问
    for i in list.iter() {
        println!("值为: {}", i);
    }
    // for循环依据下标访问
    for j in 0..5 {
        println!("list[{}] = {}", j, list[j]);
    }


    // 带退出条件的“死循环”
    let str = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut k = 0;
    loop {
        let ch = str[k];
        if ch == 'E' {
            break;
        }
        println!("\'{}\'", ch);
        k += 1;
    }

    //loop
    let mut k = 0;
    let location = loop {
        let ch = str[k];
        if ch == 'E' {
            break k;
        }
        k += 1;
    };
    println!("\'E\'的索引为{}", location);
}
