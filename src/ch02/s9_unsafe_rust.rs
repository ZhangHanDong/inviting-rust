//! 第二章：Rust核心概念
//! 2.8 Unsafe Rust
//! 
//! 内容包括：
//! 
//! - 什么是 Unsafe Rust ？
//!     
//! - Unsafe Rust 安全抽象
//!     - drop 检查
//!     - Unbound Lifetime
//!     - 型变
//! 
//! - 标准库 [LinkedList 源码](https://doc.rust-lang.org/stable/src/alloc/collections/linked_list.rs.html#38-43)导读
//!
//!  - Unsafe 工具集介绍



/**

    # Unsafe Rust 介绍

    示例1: Unsafe Rust 是 Safe Rust 的超集

    ```rust
        fn main(){
            unsafe {
                let mut a = "hello";
                let b = &a;
                let c = &mut a;
            }
        }
    ```

  Unsafe Rust是指，在进行以下五种操作的时候，并不会提供任何安全检查：

    - 解引用裸指针。
    - 调用unsafe的函数或方法。
    - 访问或修改可变静态变量。
    - 实现unsafe trait。
    - 读写Union联合体中的字段。

    解引用裸指针

    - Rust提供了*const T（不变）和*mut T（可变）两种指针类型。因为这两种指针和C语言中的指针十分相近，所以叫其原生指针（Raw Pointer）。
    
    原生指针具有以下特点：
    - 并不保证指向合法的内存。比如很可能是一个空指针。
    - 不能像智能指针那样自动清理内存。需要像C语言那样手动管理内存。
    - 没有生命周期的概念，也就是说，编译器不会对其提供借用检查。
    - 不能保证线程安全。

     可见，原生指针并不受Safe Rust提供的那一层“安全外衣”保护，所以也被称为“裸指针”。所以，在对裸指针进行解引用操作的时候，属于不安全行为。


    Unsafe语法  

    通过unsafe关键字和unsafe块就可以使用Unsafe Rust，它们的作用如下：

    - unsafe关键字，用于标记（或者说声明）函数、方法和trait。
    - unsafe块，用于执行Unsafe Rust允许的五种操作。

    查看标准库String中的 unsafe 函数[from_utf8_unchecked](https://doc.rust-lang.org/stable/std/str/fn.from_utf8_unchecked.html)，看看为什么是Unsafe的。

    这里最大的风险在于，如果一个函数存在违反“契约”的风险，而开发者并没有使用unsafe关键字将其标记，那该函数就很可能会成为Bug的温床。
    被unsafe关键字标记的不安全函数或方法，只能在unsafe块中被调用。

    

    示例2:

    ```rust
    static mut COUNTER: u32 = 0;
    fn main() {
        let inc = 3;
        unsafe {
            COUNTER += inc;
            println!("COUNTER: {}", COUNTER);
        }
    }
    ```

    Safe Rust 是基于很多 Unsafe Rust 实现的，那么 Safe Rust 凭什么 Safe ？

*/
pub fn unsafe_intro(){}

/**

    # 安全抽象

    什么叫安全抽象？ 最简单的示例：

    ```rust
    fn unbound_lifetime_foo<'a>(input: *const u32) -> &'a u32 {
        unsafe {
            return &*input
        }
    }

    fn normal_foo<'a>(input: &'a u32) -> &'a u32 {
        &input
        
    }

    fn main() {
        let x;
        { // -----------------------------------------------------------------------------------  `y` lifetime start
            // unbound lifetime broken lifetime
            let y = 7;
            x = unbound_lifetime_foo(&y);
            
            // normal lifetime will error: error[E0597]: `y` does not live long enough
            // let y = 8;
            // x = normal_foo(&y);
        } // ----------------------------------------------------------------------------------- `y` lifetime end
        println!("hello: {}", x);
    }
    ```

    示例2:

    ```rust
    pub fn insert(&mut self, index: usize, element: T) {
        let len = self.len();
        // 通过该断言保证了数组不能越界
        assert!(index <= len);
        // 通过判断长度是否达到容量极限来决定是否进行扩容
        if len == self.buf.cap() {
            self.reserve(1);
        }
        unsafe {
            {
                let p = self.as_mut_ptr().offset(index as isize);
                ptr::copy(p, p.offset(1), len - index);
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
    }
    ```


    ### Drop check 
    
    示例1：正常的drop check

    ```rust
    #![allow(unused)]
    #![allow(unused)]
    #![feature(alloc_layout_extra)]
    #![feature(dropck_eyepatch)]
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::fmt;
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;

    #[derive(Copy, Clone, Debug)]

    enum State {
        InValid,
        Valid,
    }

    #[derive(Debug)]
    struct Hello<T: fmt::Debug>(&'static str, T, State);
    impl<T: fmt::Debug> Hello<T> {
        fn new(name: &'static str, t: T) -> Self {
            Hello(name, t, State::Valid)
        }
    }
    impl<T: fmt::Debug> Drop for Hello<T> {
        fn drop(&mut self) {
            println!("drop Hello({}, {:?}, {:?})", self.0, self.1, self.2);
            self.2 = State::InValid;
        }
    }
    struct WrapBox<T> {
        v: Box<T>,
    }
    impl<T> WrapBox<T> {
        fn new(t: T) -> Self {
            WrapBox { v: Box::new(t) }
        }
    }
    fn f1() {
        let x;
        let y;
        x = Hello::new("x", 13);
        y = WrapBox::new(Hello::new("y", &x));
    }

    struct MyBox<T> {
        v: *const T,
    }
    impl<T> MyBox<T> {
        fn new(t: T) -> Self {
            unsafe {
                let p = System.alloc(Layout::array::<T>(1).unwrap());
                let p = p as *mut T;
                ptr::write(p, t);
                MyBox {
                    v: p, 
                }
            }
        }
    }


    impl< T> Drop for MyBox<T> {
        fn drop(&mut self) {
            unsafe {
                let p = self.v as *mut _;
                System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
            }
        }
    }


    fn f2() {
        {
            let (x1, y1);
            x1 = Hello::new("x1", 13);
            y1 = MyBox::new(Hello::new("y1", &x1));
        }
        {
            let (y2,x2 ); // 此处交换，会报错，注意编译错误
            x2 = Hello::new("x2", 13);
            y2 = MyBox::new(Hello::new("y2", &x2));
        }
    }

    fn main() {
        f1();
        f2();
    }

    ```

    使用 改进：

    ```rust
    #![allow(unused)]
    #![allow(unused)]
    #![feature(alloc_layout_extra)]
    #![feature(dropck_eyepatch)]
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::fmt;
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;

    #[derive(Copy, Clone, Debug)]

    enum State {
        InValid,
        Valid,
    }

    #[derive(Debug)]
    struct Hello<T: fmt::Debug>(&'static str, T, State);
    impl<T: fmt::Debug> Hello<T> {
        fn new(name: &'static str, t: T) -> Self {
            Hello(name, t, State::Valid)
        }
    }
    impl<T: fmt::Debug> Drop for Hello<T> {
        fn drop(&mut self) {
            println!("drop Hello({}, {:?}, {:?})", self.0, self.1, self.2);
            self.2 = State::InValid;
        }
    }
    struct WrapBox<T> {
        v: Box<T>,
    }
    impl<T> WrapBox<T> {
        fn new(t: T) -> Self {
            WrapBox { v: Box::new(t) }
        }
    }
    fn f1() {
        let x;
        let y;
        x = Hello::new("x", 13);
        y = WrapBox::new(Hello::new("y", &x));
    }

    struct MyBox<T> {
        v: *const T,
    }
    impl<T> MyBox<T> {
        fn new(t: T) -> Self {
            unsafe {
                let p = System.alloc(Layout::array::<T>(1).unwrap());
                let p = p as *mut T;
                ptr::write(p, t);
                MyBox {
                    v: p, 
                }
            }
        }
    }


    unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
        fn drop(&mut self) {
            unsafe {
                let p = self.v as *mut _;
                System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
            }
        }
    }

    fn f2() {
        {
            let (x1, y1);
            x1 = Hello::new("x1", 13);
            y1 = MyBox::new(Hello::new("y1", &x1));
        }
        {
            let (y2,x2 ); // 此处改变
            x2 = Hello::new("x2", 13);
            y2 = MyBox::new(Hello::new("y2", &x2));
        }
    }

    fn main() {
        f1();
        f2();
    }

    ```

    使用 PhantomData 防止出现 UB:

    ```rust
    #![allow(unused)]
    #![allow(unused)]
    #![feature(alloc_layout_extra)]
    #![feature(dropck_eyepatch)]
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::fmt;
    use std::marker::PhantomData;
    use std::mem;
    use std::ptr;

    #[derive(Copy, Clone, Debug)]

    enum State {
        InValid,
        Valid,
    }

    #[derive(Debug)]
    struct Hello<T: fmt::Debug>(&'static str, T, State);
    impl<T: fmt::Debug> Hello<T> {
        fn new(name: &'static str, t: T) -> Self {
            Hello(name, t, State::Valid)
        }
    }
    impl<T: fmt::Debug> Drop for Hello<T> {
        fn drop(&mut self) {
            println!("drop Hello({}, {:?}, {:?})", self.0, self.1, self.2);
            self.2 = State::InValid;
        }
    }
    struct WrapBox<T> {
        v: Box<T>,
    }
    impl<T> WrapBox<T> {
        fn new(t: T) -> Self {
            WrapBox { v: Box::new(t) }
        }
    }
    fn f1() {
        let x;
        let y;
        x = Hello::new("x", 13);
        y = WrapBox::new(Hello::new("y", &x));
    }

    struct MyBox<T> {
        v: *const T,
        // _pd: PhantomData<T>,
    }
    impl<T> MyBox<T> {
        fn new(t: T) -> Self {
            unsafe {
                let p = System.alloc(Layout::array::<T>(1).unwrap());
                let p = p as *mut T;
                ptr::write(p, t);
                MyBox {
                    v: p, 
                    // _pd: Default::default()
                }
            }
        }
    }


    unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::read(self.v); // 此处新增，出现UB (use after free,UAF)
                let p = self.v as *mut _;
                System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
            }
        }
    }

    fn f2() {
        {
            let (x1, y1);
            x1 = Hello::new("x1", 13);
            y1 = MyBox::new(Hello::new("y1", &x1));
        }
        {
            let (y2,x2 ); // 此处改变
            x2 = Hello::new("x2", 13);
            y2 = MyBox::new(Hello::new("y2", &x2));
        }
    }

    fn main() {
        f1();
        f2();
    }

    ```



    示例：型变

    在一门程序设计语言的类型系统中，一个类型规则或者类型构造器是：

    - 协变（covariant），如果它保持了子类型序关系≦。该序关系是：子类型≦基类型。
    - 逆变（contravariant），如果它逆转了子类型序关系。
    - 不变（invariant），如果上述两种均不适用。

    Rust 里唯一的类型父子关系是生命周期：`'a: 'b` 。比如，`'static: 'a` ，并且默认都是协变，唯一逆变的地方在 `fn(T)`

    `'static: 'a` 对应： `子类型: 父类型`。

    - 协变： 能用 'a 的地方，也可以用 'static。
    - 逆变： 能用 'static 的地方，可以可以用 'a。

    规则：

    PhantomData规则：

    - PhantomData，在T上是协变。 
    - PhantomData<&'a T>，在'a 和T上是协变。 
    - PhantomData<&'a mut T>，在'a上是协变，在T上是不变。 
    - PhantomData<*const T>，在T上是协变。 
    - PhantomData<*mut T>，在T上不变。 
    - PhantomData<fn(T)>，在T上是逆变，如果以后语法修改的话，会成为不变。 
    - PhantomData<fn() -> T>，在T上是协变。 
    - PhantomData<fn(T) -> T>，在T上是不变。 
    - PhantomData<Cell<&'a ()>>，在'a上是不变。

    ```rust
    // 协变类型
    struct MyCell<T> {
        value: T,
    }
    impl<T: Copy> MyCell<T> {
        fn new(x: T) -> MyCell<T> {
            MyCell { value: x }
        }
        fn get(&self) -> T {
            self.value
    }
    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            ptr::write(&self.value as *const _ as *mut _, value);
        }
    }
    }

    fn step1<'a>(r_c1: &MyCell<&'a i32>) {
        let val: i32 = 13;
        step2(&val, r_c1); // step2函数执行完再回到step1
        println!("step1 value: {}", r_c1.value);
    } // step1调用完，栈帧将被清理，val将不复存在，&val将成为悬垂指针

    fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
        r_c2.set(r_val);
    }
    static X: i32 = 10;
    fn main() {
    let cell = MyCell::new(&X);
    step1(&cell);
    println!("  end value: {}", cell.value); //此处 cell.value的值将无法预期，UB风险
    }

    ```

    Basic usage: 修改MyCell 类型为不变

    解决上面示例UB的问题，编译将报错，因为安全检查生效了，成功阻止了UB风险

    ```rust
    use std::marker::PhantomData;
    struct MyCell<T> {
        value: T,
        mark: PhantomData<fn(T)> , //通过PhantomData<fn(T)>将MyCell<T>改为逆变类型
    }
    impl<T: Copy> MyCell<T> {
        fn new(x: T) -> MyCell<T> {
            MyCell { value: x , mark: PhantomData}
        }
    fn get(&self) -> T {
        self.value
    }
    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            ptr::write(&self.value as *const _ as *mut _, value);
        }
    }
    }
    fn step1<'a>(r_c1: &MyCell<&'a i32>) {
        let val: i32 = 13;
        step2(&val, r_c1); // error[E0597]: `val` does not live long enough
        println!("step1 value: {}", r_c1.value);
    } // step1调用完，栈帧将被清理，val将不复存在，&val将成为悬垂指针

    fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
        r_c2.set(r_val);
    }
    static X: i32 = 10;
    fn main() {
        let cell = MyCell::new(&X);
        step1(&cell);
        println!("  end value: {}", cell.value);
    }
    ```
    
    示例：逆变

    ```rust
    #![allow(unused)]
    trait A {
        fn foo(&self, s: &'static str);
    }
    struct B;
    impl A for B {
        fn foo(&self, s: &str){
            println!("{:?}", s);
        }
    }
    impl B{
    fn foo2(&self, s: &'static str){
        println!("{:?}", s);
    }
    }
    fn main() {
        B.foo("hello");
        let s = "hello".to_string();
        B.foo2(&s)
    }

    ```

    示例2:

    ```rust
    fn foo(input: &str)  {
        println!("{:?}", input);               
    }
    fn bar(f: fn(&'static str), v: &'static str) {
        (f)(v);
    }
    fn main(){
        let v : &'static str = "hello";
        bar(foo, v);
    }
    ```

*/
pub fn security_abstract(){}


/**

    # 其他介绍

    - [NonNull](https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html)
    - [Play](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2015&gist=d95a1c5ec1a8c4279f366bb044ad6202)
    - [LinkedList](https://doc.rust-lang.org/nightly/src/alloc/collections/linked_list.rs.html#39-44)
    - [MaybeUninit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
    - 推荐阅读：[Unsafe Rust: How and when (not) to use it](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)
    - [rustsec advisories](https://rustsec.org/advisories/)
*/
pub fn nonnull(){}