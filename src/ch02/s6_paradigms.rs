//! 第二章：Rust核心概念
//! 2.5 编程范式：面向编译器编程
//!
//! 讨论：
//!
//! 1. Rust 是 FP 语言吗？
//! 2. Rust 是 OOP 语言吗？
//! 3. 如果都不是，那 Rust 是面向啥的语言 ？ 面向编译器。
//!
//!

/**

    ### Rust 是面向对象语言吗？

    OOP style:

    1. 支持面向接口编程
    2. 支持封装
    3. 不支持继承(但可以模拟)

    ```
    interface Foo {}
    class Bar: Foo {
        //Implement Foo here
    }
    ```

    ```rust
    trait Foo {}
    struct Bar;
    impl Bar;
    impl Foo for Bar {}
    ```

    ### Rust 是函数式语言吗？

    函数式style：
    1. 默认不可变（但也支持可变）
    2. 支持递归（但不支持尾递归优化）
    3. 函数是一等公民，高阶函数支持（有限）
    4. 和类型/ 积类型 (Option/Result)

    ```rust
    fn get_sum(mut total: u32, mut i: u32) -> u32 {
        if i > 10000000 {
            return total;
        }

        total = total.wrapping_add(i);
        i += 1;
        get_sum(total, i)
    }
    fn main() {
        let total = 0;
        let total = get_sum(total, 1);
        println!("{}", total);
    }

    ```

    curring:

    ```
    #[derive(Debug)]
    struct States<'a> {
        a: &'a i32,
        b: &'a i32,
    }

    trait Currying {
        type ReturnType: Fn(i32) -> i32;
        fn add(self) -> Self::ReturnType;
    }

    impl Currying for States<'static>{
        type ReturnType = Box<dyn Fn(i32) -> i32>;

        fn add(self) -> Self::ReturnType {
            Box::new(move|x| {
                x * self.a
            })
        }
    }

    let r_value: States = States {
        a: &100,
        b: &100
    };

    let r1 = r_value.add();
    let r2 = r1(5);

    assert_eq!(500, r2);
    ```

    ### Rust 是 面向编译器 编程的语言

    ```text
    +----------------------------------------------------+
    |                crate                               |
    |                                                    |
    |      +-----------------------------------+         |
    |      |           std                     |         |
    |      |                                   |         |
    |      |       +---------------------+     |         |
    |      |       |                     |     |         |
    |      |       |     core            |     |         |
    |      |       |    +----------+     |     |         |
    |      |       |    | compiler |     |     |         |
    |      |       |    +----------+     |     |         |
    |      |       |                     |     |         |
    |      |       +---------------------+     |         |
    |      |                                   |         |
    |      |                                   |         |
    |      +-----------------------------------+         |
    |                                                    |
    |                                                    |
    +----------------------------------------------------+

    ```

    查看 Rust 源码组织结构： [https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)

    洋葱模型：

    1. 最小内核所谓所有权和借用规则，就是编译器特性
    2. 基于最小内核开始构造了 core
    3. 基于core 构造了 std
    4. 基于 std 构造生态 crate
    5. 命令式编程为主（类 C），OOP 和 FP Style 辅助

    典型的实现：[std::cell::Cell](https://github.com/rust-lang/rust/blob/master/library/core/src/cell.rs)

    Cell 的语义：

    1. 在不可变引用的基础上构造一个安全的内部可变性
    2. 只针对实现 Copy 的类型提供 get 方法
    3. 对于 非 Copy 的类型，提供 move out 的方法

    ```rust

    #[stable(feature = "rust1", since = "1.0.0")]
    #[repr(transparent)]
    pub struct Cell<T: ?Sized> {
        value: UnsafeCell<T>,
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: ?Sized> !Sync for Cell<T> {}

    #[stable(feature = "rust1", since = "1.0.0")]
    impl<T: Copy> Clone for Cell<T> {
        #[inline]
        fn clone(&self) -> Cell<T> {
            Cell::new(self.get())
        }
    }

    impl<T: Eq + Copy> Eq for Cell<T> {}

    impl<T> Cell<T> {

        pub const fn new(value: T) -> Cell<T> {
            Cell { value: UnsafeCell::new(value) }
        }

        pub fn set(&self, val: T) {
            let old = self.replace(val);
            drop(old);
        }
    }

    impl<T: Copy> Cell<T> {
        pub fn get(&self) -> T {
            // SAFETY: This can cause data races if called from a separate thread,
            // but `Cell` is `!Sync` so this won't happen.
            unsafe { *self.value.get() }
        }
    }

    impl<T: Default> Cell<T> {
        #[stable(feature = "move_cell", since = "1.17.0")]
        pub fn take(&self) -> T {
            self.replace(Default::default())
        }
    }
    ```
*/
pub fn compiler_oriented_programming() {
    println!("Compiler-Oriented Programming")
}
