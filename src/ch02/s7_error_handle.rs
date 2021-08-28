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

/**

# 分层错误处理

错误处理：顾名思义，处理错误。既然要处理错误，那肯定是指开发者可以处理的情况。

-  Option: 「有」与「无」
-  Result:「对」与「错」

### Option

```rust
fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest = names[0];
        for name in names.iter() {
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }
        Some(shortest)
   } else {
       None
   }
}
fn show_shortest(names: Vec<&str>) -> &str {
   match get_shortest(names) {
       Some(shortest) => shortest,
       None             => "Not Found",
   }
}
fn main(){
   assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
   assert_eq!(show_shortest(Vec::new()), "Not Found");
}
```

使用 Match 在 「盒内」处理 Option

```rust
fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
    match get_shortest(names) {
        Some(shortest) => Some(shortest.len()),
        None             => None,
    }
}
fn main(){
    assert_eq!(get_shortest_length(vec!["Uku","Felipe"]),Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
}
```

使用标准库内建组合子处理：

```rust
fn double(value: f64) -> f64 {
    value * 2.
}
fn square(value: f64) -> f64 {
    value.powi(2 as i32)
}
fn inverse(value: f64) -> f64 {
    value * -1.
}
fn log(value: f64) -> Option<f64> {
   match value.log2() {
       x if x.is_normal() => Some(x),
       _                      => None
   }
}
fn sqrt(value: f64) -> Option<f64> {
   match value.sqrt() {
       x if x.is_normal() => Some(x),
       _                      => None
   }
}
fn main () {
   let number: f64 = 20.;
   let result = Option::from(number)
       .map(inverse).map(double).map(inverse)
       .and_then(log).map(square).and_then(sqrt);
   match result {
       Some(x) => println!("Result was {}.", x),
       None    => println!("This failed.")
   }
}
```


### Result

- Error trait:[https://doc.rust-lang.org/stable/std/error/trait.Error.html](https://doc.rust-lang.org/stable/std/error/trait.Error.html)
- Result Error Handle : [read-sum crate]
- `?` and [std::ops::Try](https://doc.rust-lang.org/stable/std/ops/trait.Try.html)




```rust
use std::num::ParseIntError;
// fn square(number_str: &str) -> Result<i32, ParseIntError>
// {
//    number_str.parse::<i32>().map(|n| n.pow(2))
// }
type ParseResult<T> = Result<T, ParseIntError>;
fn square(number_str: &str) -> ParseResult<i32>
{
    number_str.parse::<i32>().map(|n| n.pow(2))
}
fn main() {
    match square("10") {
        Ok(n) => assert_eq!(n, 100),
        Err(err) => println!("Error: {:?}", err),
    }
}
```


*/
pub fn error_handle() {
    println!("Error Handle!")
}

/**

### panic 的两种类型：

- Unwinding（栈展开）。
- Aborting（中止）。

Unwinding 可以使应用程序线程以相对干净的方式关闭。
回收所有分配的系统资源，正确删除所有应用程序对象，依此类推。
此外，恐慌停止在有问题的线程的边界，而不是杀死整个应用程序过程。
所有这一切意味着，如果所有对象都具有明智的析构函数，则尽管有困难，但仍可以从紧急情况中恢复应用程序。

如果你应用程序是为此目的而设计的，则可以检测到线程紧急情况并重新启动有问题的线程，希望该操作能够正确恢复。
在无法关闭应用程序的情况下，例如在关键系统中，类似于Erlang的容错方法可能是有意义的。

对于Aborting，不存在应用程序恢复的可能性。一旦某些代码中止，应用程序进程将立即终止，这意味着要实现容错功能，就需要进行更加复杂的多进程设计。
另外，由于未运行资源析构函数，因此整个系统可能处于不一致状态，这意味着重新启动应用程序可能非常不容易。

总而言之，仅应在确实不关心应用程序立即崩溃并可能破坏在崩溃过程中操作的任何硬件/操作系统状态的情况下启用Aborting恐慌。

需要了解一个事实，Rust 目前对 OOM(out of memory)对处理是直接 Aborting ，无论你如何设置Panic类型。

### 恐慌安全：

[Rust Magazine #01 security](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust_security_part1.html)
- catch_unwind


```rust

use std::panic;
fn sum(a: i32, b: i32) -> i32{
    a + b
}
fn main() {
    let result = panic::catch_unwind(|| { println!("hello!"); });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| { panic!("oh no!"); });
    assert!(result.is_err());
   println!("{}", sum(1, 2));
}
```

使用 set_hook

```rust
use std::panic;
fn sum(a: i32, b: i32) -> i32{
    a + b
}
fn main() {
    let result = panic::catch_unwind(|| { println!("hello!"); });
    assert!(result.is_ok());
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!("panic occurred '{}' at {}",
                location.file(), location.line()
            );
       } else {
            println!("can't get location information...");
       }
   }));
   let result = panic::catch_unwind(|| { panic!("oh no!"); });
   assert!(result.is_err());
   println!("{}", sum(1, 2));
}
```


*/
pub fn panic_cant_handle() {
    println!("Panic Can't Handle")
}
