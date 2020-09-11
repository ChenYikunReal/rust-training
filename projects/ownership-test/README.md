# Rust所有权

所有权有三条基本规则：
1. Rust中的每个值都有一个变量，称为其所有者
2. 一次只能有一个所有者
3. 当所有者不在程序运行范围时，该值将被删除


资源有分配就有释放，程序不能一直占用某个内存资源。
>决定资源是否浪费的关键因素就是资源有没有及时的释放。

下面的Rust程序：
```rust
fn function() {
    // 在声明以前，变量s无效
    let s = "test";
    // 这里是变量s的可用范围
}
// 变量范围已经结束，变量s无效
```

上面的Rust代码相当于下面的C语言代码：
```c
void function() {
    char *s = "test";
    free(s); // 释放 s 资源
}
```

# Rust变量与数据的交互
建议细读[此文](https://www.runoob.com/rust/rust-ownership.html)
