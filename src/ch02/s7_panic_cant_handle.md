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
