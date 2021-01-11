//! 第二章：Rust核心概念
//! 2.1 安全管理之内存安全
//! 
//! 所有权相关代码
//! 
//!  String 结构：
//!  
//!  ```text
//!  ---  
//!                   buffer
//!                  /   capacity
//!                 /   /  length
//!                /   /   /
//!              +–––+–––+–––+
//!  stack frame │ • │ 8 │ 6 │ <- my_name: String
//!              +–│–+–––+–––+
//!                │
//!              [–│–––––––– capacity –––––––––––]
//!                │
//!              +–V–+–––+–––+–––+–––+–––+–––+–––+
//!  heap        │ P │ a │ s │ c │ a │ l │   │   │
//!              +–––+–––+–––+–––+–––+–––+–––+–––+
//!              [––––––– length ––––––––]
//!   
//!  &'static str 结构：
//!              [–––––––––––]
//!              +–––+–––+
//!  stack frame │ • │ 6 │ 
//!              +–│–+–––+
//!                │                 
//!                +––+                
//!                   │
//!  preallocated   +–V–+–––+–––+–––+–––+–––+
//!  read-only      │ P │ a │ s │ c │ a │ l │
//!  memory         +–––+–––+–––+–––+–––+–––+
//!  ```


/**
    ### Rust 语义：Move 语义 与 Copy 语义

    基本数据类型： https://doc.rust-lang.org/std/index.html#primitives

    ```
    fn main(){
        // impl Copy for i32
        let a = 42;
        let b = a;
        println!("{:?}", a);  // work

        // impl Copy for &'static str
        let a = "42";
        let b = a;
        println!("{:?}", a); // work
        
        // impl !Copy for String
        let a = "42".to_string();
        // &String deref to &str
        let b : &str = &a;
        // impl Copy for &'a T
        let c = b;
        println!("{:?}", b); // work

        // impl !Copy for String
        let mut a = "42".to_string();
        // impl !Copy for &mut T
        let b : &mut str = &mut a;
        let c = b;
        // println!("{:?}", b); // don't work, b have been moved
        
        // auto impl Copy for Tuple, if all item implemented Copy trait in Tuple
        let t = (42, "42");
        let t2 = t;
        println!("{:?}", t); // work
        
        // auto impl !Copy for Tuple
        let t = (42, "42".to_string());
        let t2 = t;
        // println!("{:?}", t); // don't work, t have been moved
    }
    ```
*/
pub fn primitive_types(){ 
    println!(" Copy 和 Move 语义： 基础数据类型实现 Copy 的一些情况 ");
}


/**
    ### Rust 语义：Move 语义 与 Copy 语义

    自定义数据类型：

    ```
    // #[derive(Copy, Clone)]
    struct A;

    // #[derive(Copy, Clone)]
    struct Point(u32);

    // #[derive(Copy, Clone)]
    struct Member {
        name: &'static str,
        age: u32,
    }

    // #[derive(Copy, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    fn main(){
        let a = A;
        let b = a;
        println!("{:?}", a);  // work

        let a = Point(60);
        let b = a;
        println!("{:?}", a);  // work

        let a = Member{name: "Alex", age: "18"};
        let b = a;

        let a = Member{name: "Alex".to_string(), age: "18"};
        let b = a;
    }
    ```
*/
pub fn custom_types(){ 
    println!(" Copy 和 Move 语义： 自定义类型实现 Copy 的一些情况 ");
}


/**
    ### Rust 语义：Move 语义 与 Copy 语义

    - 理解 Copy：Clone  https://doc.rust-lang.org/std/marker/trait.Copy.html


    ```
    struct A;

    // 没用，自己实现Copy和Clone无法改变编译器默认行为
    impl Clone for A {
        fn clone(&self) -> Self {
            println!("from Custom Copy: Clone");
            *self
        }
    }

    impl Copy for A {}


    fn main(){
        let a = A;
        let b = a;
    }
    
    ```
*/
pub fn understand_copy_clone(){ 
    println!(" 理解按位复制 ： Copy ");
}


/**
    ### Rust 语义：Move 语义 与 Copy 语义

    - 理解 按位复制

    ```
    #[derive(Copy, Clone)]
    struct A(i8, i32);
    fn main() {
        let a = A(1, 2);
        let b = a; // 按位复制，复制后，b和a完全相同，包括内存对齐填充的padding部分。
        let c = A(a.0, a.1); // 逐成员复制，非按位复制，c和a的padding部分不一定相同。        
    }
    
    ```

    示例二：
    
    ```rust
    #[derive(Debug, Copy, Clone)]
    struct A {
        a: u16,
        b: u8,
        c: bool,
    }

    fn main() {
        let a = unsound_a();
        // 尝试将 Some(a) 改为 a
        let some_a = Some(a);
        
        println!("a: {:#?}", a);
        println!("some_a: {:#?}", some_a);
    }


    fn unsound_a() -> A {
        #[derive(Debug, Copy, Clone)]
        struct B {
            a: u16,
            b: u8,
            c: u8,
        }
        // 依次修改 c 的值为 0，1，2 打印输出结果
        let b = B { a: 1, b: 1, c: 1 };
        unsafe {*(&b as *const B as *const A) }
    }
    ```

    示例三：

    ```rust
    #![allow(unused_variables)]

    use std::{ptr, mem};

    fn main() {
        let mut d = String::from("cccc");
        let d_len = d.len();
        // {
            let mut c = String::with_capacity(d_len);

            unsafe {
                ptr::copy(&d, &mut c, 1);
            };
            println!("{:?}", c.as_ptr());
            // unsafe {
            //     ptr::drop_in_place(c.as_mut_ptr());
            // }
            // 注掉 drop，会产生double free，
            // 但是不注掉 drop，会产生无效指针
            mem::drop(c);
        // }

        println!("{:?}", d.as_ptr());
        d.push_str("c");
        println!("{}", d);
    }
    ```

    示例四: Copy 不一定只在栈上进行

    ```rust
    use std::cell::RefCell;

    fn main() {
        let a = Box::new(RefCell::new(1));
        let b = Box::new(RefCell::new(2));
        *b.borrow_mut() = *a.borrow();
        println!("b = {}", b.borrow());
    }
    ```
*/
pub fn understand_copy(){ 
    println!(" 理解按位复制 ： Copy ");
}



/**

    示例1: Box<T> 实现 DereMove

    ```rust
    fn main(){
        let s = Box::new("hello".to_string());
        println!("{:p}", &s);
        println!("{:p}", s.as_ptr());
        // DerefMove
        let s2 = *s;
        // println!("{:p}", s.as_ptr()); // Moved s
        println!("{:p}", s2.as_ptr());
    }
    ```

    示例二：Arc 无法 DerefMove

    https://doc.rust-lang.org/std/sync/struct.Arc.html



    ```rust
    use std::sync::Arc;

    fn main(){
        let s = Arc::new("hello".to_string());
        println!("{:p}", &s);
        println!("{:p}", s.as_ptr());
        // DerefMove Error : cannot move out of an `Arc`
        let s2 = *s;
        // println!("{:p}", s.as_ptr()); // Moved s
        println!("{:p}", s2.as_ptr());
    }
    ```

*/
pub fn understand_move(){ 
    println!(" 理解 Move 语义： 解引用Move ");
}


/**
    语义层面来理解 Clone ：显式的clone方法调用同一种语义下的两种实现
    1. String 等 引用类型的 Clone
    2. Rc/Arc 类型的 Clone
*/
pub fn understand_clone(){ 
    println!(" 理解 Move 语义： 解引用Move ");
}


/**

    示例1: Move 的本质：drop 标记

    ```rust
    fn main(){
        // impl Copy for i32
        let mut a = "42".to_string();
        let b = a; // drop(a);
        
        a = "32".to_string();
        println!("{:?}", a);
    }
    ```

    示例二：Drop 析构函数


    ```rust
    struct PrintDrop(&'static str);
    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
        }
    }
    fn main() {
        let x = PrintDrop("x");
        let y = PrintDrop("y");
    }
    ```
    
    元组：

    ```rust
    struct PrintDrop(&'static str);
        impl Drop for PrintDrop {
            fn drop(&mut self) {
                println!("Dropping {}", self.0)
        }
    }
    fn main() {
        let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
        let tup2 = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));
    }
    ```

    带panic的元组：

    ```rust
    struct PrintDrop(&'static str);
    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
    }
    }
    fn main() {
        let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
        let tup2 = (PrintDrop("x"), PrintDrop("y"), panic!());
    }

    ```
 
    结构体：

    ```rust
    struct PrintDrop(&'static str);

    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
        }
    }

    struct Foo {
        bar: PrintDrop,
        baz: PrintDrop,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo")
        }
    }

    fn main() {
        let foo = Foo {
            bar: PrintDrop("bar"),
            baz: PrintDrop("baz"),
        };
    }
    ```

    闭包：

    ```
    struct PrintDrop(&'static str);
    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
        }
    }
    fn main() {
        let z = PrintDrop("z");
        let x = PrintDrop("x");
        let y = PrintDrop("y");
        let closure = move || { y; z; x; };
    }
    ```

    闭包修改变量：

    ```
    struct PrintDrop(&'static str);
    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
        }
    }
    fn main() {
        let y = PrintDrop("y");
        let x = PrintDrop("x");
        let z = PrintDrop("z");
        let closure = move || {
            { let z_ref = &z; }
            x; y; z;
        };
    }
    ```

    示例三： 所有权 forget/ ManuallyDrop

    ```rust
    // https://doc.rust-lang.org/src/alloc/sync.rs.html#319
    impl<T> Arc<T> {
        pub fn new(data: T) -> Arc<T> {
            // Start the weak pointer count as 1 which is the weak pointer that's
            // held by all the strong pointers (kinda), see std/rc.rs for more info
            let x: Box<_> = box ArcInner {
                strong: atomic::AtomicUsize::new(1),
                weak: atomic::AtomicUsize::new(1),
                data,
            };
            // ManuallyDrop
            Self::from_inner(Box::leak(x).into())
        }

        // ...
    }

    impl<T> Weak<T> {
        pub fn into_raw(self) -> *const T {
            let result = self.as_ptr();
            mem::forget(self);
            result
        }
    }
    ```

*/
pub fn understand_drop(){ 
    println!(" 理解 Drop ");
}