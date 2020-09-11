# Rust异常处理的简单说明

1. Rust不使用`try...catch...`结构处理异常。
2. Rust没有Exception，只区分可恢复错误和不可恢复错误。对于可恢复错误用`Result<T, E>`类来处理，对于不可恢复错误使用`panic!`宏来处理。
