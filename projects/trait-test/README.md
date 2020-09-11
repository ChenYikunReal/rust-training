# Trait复杂关系的简化

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
```
可以简化为：
```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {}
```
