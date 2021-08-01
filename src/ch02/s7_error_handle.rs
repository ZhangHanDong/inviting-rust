//! 第二章：Rust核心概念
//! 2.6 错误处理
//!
//! 1. 类型系统保证函数契约
//! 2. 断言用于防御
//! 3. Option<T> 消除空指针失败
//! 4. Result<T, E> 传播错误
//! 5. Panic 恐慌崩溃
//!

/**
# 消除失败

1. 类型系统保证函数契约

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    sum(1u32, 2u32); // 违反契约，报错
}
```

2. 断言用于防御

```rust

fn extend_vec(v: &mut Vec<i32>, i: i32) {
    assert!(v.len() == 5);
    v.push(i);
}
fn main() {
    let mut vec = vec![1, 2, 3];
    extend_vec(&mut vec, 4);
    extend_vec(&mut vec, 5);
    extend_vec(&mut vec, 6); // panic!
}
```
*/
pub fn elim_failure() {
    println!("Eliminate Failure!");
}

#[doc = include_str!("s7_error_handle.md")]
pub fn error_handle() {
    println!("Error Handle!")
}

#[doc = include_str!("s7_panic_cant_handle.md")]
pub fn panic_cant_handle() {
    println!("Panic Can't Handle")
}
