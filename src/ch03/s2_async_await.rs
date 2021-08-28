//! 第三章：Rust 异步编程概念
//!
//! # 3.2 异步 编程 模型
//!
//! - [smol](https://github.com/smol-rs/smol)
//! - Future
//! - 生成器 与 协程
//! - Pin 与 UnPin
//! - Async/Await
//!
//!

/**

    让我们先从建立异步编程模型的整体概念框架开始，先不深入细节。

    **Rust 提供的异步并发相比于其他语言有什么特点？**

    1. Rust 语言只是提供一个零成本的异步编程抽象，而不内置运行时。
    2. 基于 Generator 实现的 Future，在 Future 基础上 提供 async/await 语法糖。本质是一个状态机。

    查看 README 和其他编程语言比较的图示。

    **为什么需要异步？**

    1. 对极致性能的追求。
    2. 对编程体验的追求。

    **异步编程模型发展阶段：**

    1. Callback
    2. Promise/Future
    3. async/await

    可在项目 README 查看回调地狱示例图。

    ```text
    A
    +------+
    |      |
    |      |
    |      +-------------------+
    |      |                   |
    |      |                   |
    |      |                  Bv
    +------+                +-----+
    |      |                |     |
    |      |                | do1 |
    |      |                |     |
    |      |                +-----+
    |      |                |     |
    |      |                | do2 |
    |      |                +-+---+
    |      |                  |
    |      |                  |
    | A    |                  |
    +------+                  |
    |      |                  |
    |      |                  |
    |      |                  |
    |      | <----------------+
    |      |
    |      |
    |      |
    +------+

    ```

    早期 Rust 异步写法示意：

    ```rust
    let future = id_rpc(&my_server).and_then(|id| {
        get_row(id)
    }).map(|row| {
        json::encode(row)
    }).and_then(|encoded| {
        write_string(my_socket, encoded)
    });
    ```

    这样写会存在大量内嵌 Future，开发体验不好。

    引入 async/await 之后：

    ```rust
    let id = id_rpc(&my_server).await;
    let row = get_row(id).await;
    let encoded = json::encode(row);
    write_string(my_socket, encoded).await;
    ```

    拥有了和同步代码一致的体验。

    **异步任务可看作是一种绿色线程**

    查看 README 相关图示

    可以说，异步任务的行为是模仿 线程 来抽象。

    1. 线程在进程内，异步任务在线程内。
    2. 线程可被调度切换（Linux默认抢占式），异步任务也可以被调度（协作式而非抢占式）。区别在于，异步任务只在用户态，没有线程的上下文切换开销。
    3. 线程有上下文信息，异步任务也有上下文信息。
    4. 线程间可以通信，异步任务之间也可以通信。
    5. 线程间有竞争，异步任务之间也有竞争。

    整个异步编程概念，包括异步语法、异步运行时都是围绕如何建立这种「绿色线程」抽象而成的。

*/
pub fn a_async_intro() {}

/**

    # Future 和 Futures-rs 概要

    - [Future](https://doc.rust-lang.org/std/future/index.html) and [task](https://doc.rust-lang.org/std/task/index.html)
    - [futures-rs](https://github.com/rust-lang/futures-rs)



*/

pub fn b_futures_simple_intro() {}

/**

    通过实现一个简单的 async/await echo server 来理解 Futures

    代码示例参见本仓库内：async-echo-server

    通过代码实现 简单的异步运行时（executor+reactor）

*/

pub fn c_async_await_echo() {}

/**

    # 深入理解 Future 和 Futures-rs


    回顾 Rust 异步 task 模型

    ```text
        +------------------------------------------------------------------+
        |                                                                  |
        |    +--------------------------------------------------------+    |
        |    |                                                        |    |
        |    |   +-------------------------------------------------+  |    |
        |    |   |-------------+ +----------+ +--------------+     |  |    |
        |    |   || futureobj  | | futureobj| |  futureobj   |     |  |    |
        |    |   +-------------+ +----------+ +--------------+     |  |    |
        |    |   | 协 程  task                                      |  |    |
        |    |   +-------------------------------------------------+  |    |
        |    |                                                        |    |
        |    | 线 程                                                   |    |
        |    +--------------------------------------------------------+    |
        |                                                                  |
        |                                                                  |
        |    +--------------------------------------------------------+    |
        |    |  +--------------------------------------------------+  |    |
        |    |  |                                                  |  |    |
        |    |  |   +------------+ +-------------------------------+  |    |
        |    |  |   | futureobj  | |  futureobj      || futureobj ||  |    |
        |    |  |   +------------+ +-------------------------------+  |    |
        |    |  |  协 程 task                                      |  |    |
        |    |  +--------------------------------------------------+  |    |
        |    | 线 程                                                   |    |
        |    +--------------------------------------------------------+    |
        |                                                                  |
        | 进  程                                                            |
        +------------------------------------------------------------------+


    ```


    1. 理解 leaf-futures vs Non-leaf-futures (async/await)
    2. 理解 Waker：

    > 当事件源注册该Future将在某个事件上等待时，它必须存储唤醒程序，以便以后可以调用唤醒来开始唤醒阶段。
    > 为了引入并发性，能够同时等待多个事件非常重要，因此唤醒器不可能由单个事件源唯一拥有。 结果，Waker类型需要是实现 Clone 的。

    - [https://doc.rust-lang.org/std/task/struct.Waker.html](https://doc.rust-lang.org/std/task/struct.Waker.html)


    3. 理解并发（waker 并发 和 poll 并发）


    深入 Futures-rs:

    - [Future](https://doc.rust-lang.org/std/future/index.html) and [task](https://doc.rust-lang.org/std/task/index.html)
    - [futures-rs](https://github.com/rust-lang/futures-rs)
    - [futures-lite](https://github.com/smol-rs/futures-lite)


*/

pub fn d_futures_rs() {}

/**


    # 异步实现细节：生成器 与 协程

    ## 历史

    处理异步事件的三种方式：
    - Callback
    - Promise/Future
    - async/await

    async/await 是目前体验最好的方式，Rust 要支持它并不容易。

    ## async/await 语法介绍

    参考：[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)

    `async` 两种用法：`async fn ` 函数 和 `async {}` 块。

    ```rust
    // async 函数，真正会返回 `Future<Output = u8>`，而不是表面看上去的 `u8`
    async fn foo() -> u8 { 5 }

    // async 块用法，返回 `impl Future<Output = u8>`
    fn bar() -> impl Future<Output = u8> {
        // 这里 `async` 块返回  `impl Future<Output = u8>`
        async {
            let x: u8 = foo().await;
            x + 5
        }
    }
    ```

    `await` 将暂停当前函数的执行，直到执行者将 Future 结束为止。这为其他 Future 任务提供了计算的机会。


    ## 生成器

    Future 底层实现依赖于 生成器。 `async/await` 对应底层生成器 `resume/yield` 。

    ```rust
    #![feature(generators, generator_trait)]
    use std::ops::Generator;
    use std::pin::Pin;

    fn main() {
        let mut gen = || {
            yield 1;
            yield 2;
            yield 3;
            return 4;
        };
        // for _ in 0..4 {
        //     // 为了给嵌入式支持异步，多传入了一个空的unit给resume方法
        //     let c = Pin::new(&mut gen).resume(());
        //     println!("{:?}", c);
        // }


        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
    }

    ```

    生成等价代码：

    ```rust
    #![allow(unused)]
    #![feature(generators, generator_trait)]
    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    enum __Gen {
        // (0) 初始状态
        Start,
        // (1) resume方法执行以后
        State1(State1),
        // (2) resume方法执行以后
        State2(State2),
        // (3) resume方法执行以后
        State3(State3),
        // (4) resume方法执行以后，正好完成
        Done
    }


    struct State1 { x: u64 }
    struct State2 { x: u64 }
    struct State3 { x: u64 }

    impl Generator for __Gen {
        type Yield = u64;
        type Return = u64;

        fn resume(self: Pin<&mut Self>, _: ()) -> GeneratorState<u64, u64> {
            let mut_ref = self.get_mut();
            match std::mem::replace(mut_ref, __Gen::Done) {
                __Gen::Start => {
                    *mut_ref = __Gen::State1(State1{x: 1});
                    GeneratorState::Yielded(1)
                }
                __Gen::State1(State1{x: 1}) => {
                    *mut_ref  = __Gen::State2(State2{x: 2});
                    GeneratorState::Yielded(2)
            }
                __Gen::State2(State2{x: 2}) => {
                    *mut_ref = __Gen::State3(State3{x: 3});
                    GeneratorState::Yielded(3)
                }
                __Gen::State3(State3{x: 3}) => {
                    *mut_ref  = __Gen::Done;
                    GeneratorState::Complete(4)
                }
                _ => {
                    panic!("generator resumed after completion")
                }
            }
        }
    }

    fn main(){
        let mut gen = {
            __Gen::Start
        };

        for _ in 0..4 {
            println!("{:?}", unsafe{ Pin::new(&mut gen).resume(())});
        }
    }
    ```

    生成器基本用法：

    ```rust

    #![allow(unused)]
    #![feature(generators, generator_trait)]
    use std::pin::Pin;

    use std::ops::Generator;

    pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
        move || {
            for x in 0..limit {
                yield x;
            }
            return limit;
        }
    }
    fn main(){
        let a = 10;
        let mut b = up_to(a);
        unsafe {
            for _ in 0..=10{
                let c = Pin::new(&mut b).resume(());
                println!("{:?}", c);
            }
        }
    }

    ```

    生成器变身为迭代器：

    ```rust
    #![allow(unused)]
    #![feature(generators, generator_trait)]
    use std::pin::Pin;

    use std::ops::{Generator, GeneratorState};

    pub fn up_to() -> impl Generator<Yield = u64, Return = ()> {
        move || {
            let mut x = 0;
            loop {
                x += 1;
                yield x;
            }
            return ();
        }
    }
    fn main(){
        let mut gen = up_to();
        unsafe {
        for _ in 0..10{
            match Pin::new(&mut gen).resume(()) {
                GeneratorState::Yielded(i) => println!("{:?}", i),
                _ => println!("Completed"),
            }
        }
        }
    }
    ```

    生成器变身为 Future:

    ```rust

    #![allow(unused)]
    #![feature(generators, generator_trait)]

    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    pub fn up_to(limit: u64) -> impl Generator<Yield = (), Return = Result<u64, ()>> {
        move || {
            for x in 0..limit {
                yield ();
            }
            return Ok(limit);
        }
    }
    fn main(){
        let limit = 2;
        let mut gen = up_to(limit);
        unsafe {
        for i in 0..=limit{
            match Pin::new(&mut gen).resume(()) {
                GeneratorState::Yielded(v) => println!("resume {:?} : Pending", i),
                GeneratorState::Complete(v) => println!("resume {:?} : Ready", i),
            }
        }
        }
    }
    ```


    跨 yield 借用会报错：

    ```rust
    #![allow(unused)]
    #![feature(generators, generator_trait)]

    use std::ops::Generator;
    use std::pin::Pin;

    pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
        move || {
            let a = 5;
            let ref_a = &a;
            for x in 0..limit {
                yield x;
                if x == 5{
                    yield *ref_a;
                }
            }
            return limit;
        }
    }
    fn main(){
        let a = 10;
        let mut b = up_to(a);
        unsafe {
            for _ in 0..=10{
                let c = Pin::new(&mut b).resume(());
                println!("{:?}", c);
            }
        }
    }
    ```


    自引用结构：

    ```rust
    #![feature(generators, generator_trait)]
    use std::ops::Generator;
    use std::pin::Pin;
    fn main(){
        let mut generator = move || {
            let to_borrow = String::from("Hello");
            let borrowed = &to_borrow;
            // error[E0626]: borrow may still be in use when generator yields
            yield borrowed.len();
            println!("{} world!", borrowed);
        };
    }

    ```

    模拟底层实现 generator ：

    ```rust
    #![allow(unused)]
    #![feature(never_type)] // Force nightly compiler to be used in playground
    // by betting on it's true that this type is named after it's stabilization date...
    pub fn main() {
        let mut gen = GeneratorA::start();
        let mut gen2 = GeneratorA::start();

        if let GeneratorState::Yielded(n) = gen.resume() {
            println!("Got value {}", n);
        }

        // std::mem::swap(&mut gen, &mut gen2); // <--- Big problem!

        if let GeneratorState::Yielded(n) = gen2.resume() {
            println!("Got value {}", n);
        }

        // This would now start gen2 since we swapped them.
        if let GeneratorState::Complete(()) = gen.resume() {
            ()
        };

        if let GeneratorState::Complete(()) = gen2.resume() {
            ()
        };
    }
    enum GeneratorState<Y, R> {
        Yielded(Y),
        Complete(R),
    }

    trait Generator {
        type Yield;
        type Return;
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
    }

    enum GeneratorA {
        Enter,
        Yield1 {
            to_borrow: String,
            borrowed: *const String,
        },
        Exit,
    }

    impl GeneratorA {
        fn start() -> Self {
            GeneratorA::Enter
        }
    }
    impl Generator for GeneratorA {
        type Yield = usize;
        type Return = ();
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
                match self {
                GeneratorA::Enter => {
                    let to_borrow = String::from("Hello");
                    let borrowed = &to_borrow;
                    let res = borrowed.len();
                    *self = GeneratorA::Yield1 {to_borrow, borrowed: std::ptr::null()};

                    // We set the self-reference here
                    if let GeneratorA::Yield1 {to_borrow, borrowed} = self {
                        *borrowed = to_borrow;
                    }

                    GeneratorState::Yielded(res)
                }

                GeneratorA::Yield1 {borrowed, ..} => {
                    let borrowed: &String = unsafe {&**borrowed};
                    println!("{} world", borrowed);
                    *self = GeneratorA::Exit;
                    GeneratorState::Complete(())
                }
                GeneratorA::Exit => panic!("Can't advance an exited generator!"),
            }
        }
    }
    ```
    上面代码在 Safe Rust 下是存在问题的，有 UB 风险。


    用 Pin 修正：

    ```rust
    #![feature(generators, generator_trait)]
    use std::ops::{Generator, GeneratorState};


    pub fn main() {
        // 使用 static 关键字创建 immovable 生成器
        let gen1 = static || {
            let to_borrow = String::from("Hello");
            let borrowed = &to_borrow;
            yield borrowed.len();
            println!("{} world!", borrowed);
        };

        let gen2 = static || {
            let to_borrow = String::from("Hello");
            let borrowed = &to_borrow;
            yield borrowed.len();
            println!("{} world!", borrowed);
        };

        let mut pinned1 = Box::pin(gen1);
        let mut pinned2 = Box::pin(gen2);

        if let GeneratorState::Yielded(n) = pinned1.as_mut().resume(()) {
            println!("Gen1 got value {}", n);
        }

        if let GeneratorState::Yielded(n) = pinned2.as_mut().resume(()) {
            println!("Gen2 got value {}", n);
        };

        let _ = pinned1.as_mut().resume(());
        let _ = pinned2.as_mut().resume(());
    }

    ```


    第三方库 genawaiter：stable rust 实现 generator 。

    [https://github.com/whatisaphone/genawaiter](https://github.com/whatisaphone/genawaiter)

*/
pub fn e_generator() {}

/**


    ### 前奏


    Safe Rust 无法构建自引用结构体：

    ```rust
    struct SelfReferential<'a> {
        a: String,
        b: &'a String,
    }

    fn main() {
        let a = String::from("Hello");
        let _sr = SelfReferential { a, b: &a }; // error: borrow of moved value: `a`
    }
    ```

   另一个示例：即便使用 `Box<T>` 放到堆上，也存在风险：

    ```rust
    struct SelfReferential {
        self_ptr: *const Self,
    }

    fn main() {
        let mut heap_value = Box::new(SelfReferential {
            self_ptr: 0 as *const _,
        });
        let ptr = &*heap_value as *const SelfReferential;
        heap_value.self_ptr = ptr;
        println!("heap value at: {:p}", heap_value);
        println!("internal reference: {:p}", heap_value.self_ptr);

        // 风险代码
        let stack_value = mem::replace(&mut *heap_value, SelfReferential {
            self_ptr: 0 as *const _,
            _pin: PhantomPinned,
        });
        println!("value at: {:p}", &stack_value);
        println!("internal reference: {:p}", stack_value.self_ptr);
    }

    ```


    使用指针来构建，但是有安全风险：

    ```rust
    #[derive(Debug)]
    struct SelfReferential {
        a: String,
        b: *const String,
    }

    impl SelfReferential {
        fn new(txt: &str) -> Self {
            Self {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            unsafe {&*(self.b)}
        }
    }

    fn main() {
        let mut sf1 = SelfReferential::new("test1");
        sf1.init();
        let mut sf2 = SelfReferential::new("test2");
        sf2.init();

        println!("a: {}, b: {}", sf1.a(), sf1.b());
        println!("a: {:p}, b: {:p}, t: {:p}", &(sf1.a), sf1.b, &(sf2.a));
        // 使用swap()函数交换两者，这里发生了move
        std::mem::swap(&mut sf1, &mut sf2);


        sf1.a = "I've totally changed now!".to_string();
        println!("a: {}, b: {}", sf2.a(), sf2.b());
        println!("a: {:p}, b: {:p}, t: {:p}", &(sf1.a), sf1.b, &(sf2.a) ) ;
    }
    ```

    Pin 到栈上：

    ```rust
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    #[derive(Debug)]
    struct SelfReferential {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl SelfReferential {
        fn new(txt: &str) -> Self {
            Self {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // This makes our type `!Unpin`
            }
        }

        fn init<'a>(self: Pin<&'a mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a<'a>(self: Pin<&'a Self>) -> &'a str {
            &self.get_ref().a
        }

        fn b<'a>(self: Pin<&'a Self>) -> &'a String {
            unsafe { &*(self.b) }
        }
    }

    pub fn main() {
        let mut sf1 = unsafe { Pin::new_unchecked(&mut SelfReferential::new("test1")) };
        SelfReferential::init(sf1.as_mut());

        let mut sf2 = unsafe { Pin::new_unchecked(&mut SelfReferential::new("test2")) };
        SelfReferential::init(sf2.as_mut());

        println!("a: {}, b: {}", SelfReferential::a(sf1.as_ref()), SelfReferential::b(sf1.as_ref()));
        std::mem::swap(sf1.get_mut(), sf2.get_mut());
        println!("a: {}, b: {}", SelfReferential::a(sf2.as_ref()), SelfReferential::b(sf2.as_ref()));
    }

    ```

    一个 `Pin<&mut T>` 必须在被引用的T的整个生命周期被保持 pinned，这对于栈上的变量很难确认。
    为了帮助处理这类问题，就有了像[pin-utils](https://docs.rs/pin-utils)这样的 crate。


    栈上 vs 堆上：

    ```rust
    fn main(){

        let s = "hello".to_string();
        let p = s.as_str();
        println!("{:p}", p);
        println!("{:p}", s.as_ptr());
        println!("{:p}", &s);
    }
    ```


    Pin 到 堆上：

    ```rust
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    #[derive(Debug)]
    struct SelfReferential {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl SelfReferential {
        fn new(txt: &str) -> Pin<Box<Self>> {
            let t = Self {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

            boxed
        }

        fn a<'a>(self: Pin<&'a Self>) -> &'a str {
            &self.get_ref().a
        }

        fn b<'a>(self: Pin<&'a Self>) -> &'a String {
            unsafe { &*(self.b) }
        }
    }

    pub fn main() {
        let mut sf1 = SelfReferential::new("test1");
        let mut sf2 = SelfReferential::new("test2");

        println!("a: {}, b: {}",sf1.as_ref().a(), sf1.as_ref().b());
        std::mem::swap(sf1.get_mut(), sf1.get_mut());
        // std::mem::swap(&mut *sf1, &mut *sf2);
        println!("a: {}, b: {}",sf2.as_ref().a(), sf2.as_ref().b());
    }

    ```

    另外一个示例：

    ```rust
    use std::mem;
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    struct SelfReferential {
        self_ptr: *const Self,
        _pin: PhantomPinned,
    }

    fn main() {
        let mut heap_value = Box::pin(SelfReferential {
            self_ptr: 0 as *const _,
            _pin: PhantomPinned,
        });
        let ptr = &*heap_value as *const SelfReferential;

        // 这是安全的，因为修改结构体字段不会让结构体发生move
        unsafe {
            let mut_ref = Pin::as_mut(&mut heap_value);
            Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
        }

        println!("heap value at: {:p}", heap_value);
        println!("internal reference: {:p}", heap_value.self_ptr);

        // 有效阻止了下面风险代码的发生

        let stack_value = mem::replace(&mut *heap_value, SelfReferential {
            self_ptr: 0 as *const _,
            _pin: PhantomPinned,
        });
        println!("value at: {:p}", &stack_value);
        println!("internal reference: {:p}", stack_value.self_ptr);
    }




    ```

    ### 异步实现细节：Pin 与 UnPin

    async/await 的要支持引用，就必须支持自引用结构。

    Rust 源码内部：[https://github.com/rust-lang/rust/blob/master/library/core/src/future/mod.rs](https://github.com/rust-lang/rust/blob/master/library/core/src/future/mod.rs)

    ```rust
    // From Generator to Future
    pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
    where
        T: Generator<ResumeTy, Yield = ()>,
    {
        #[rustc_diagnostic_item = "gen_future"]
        struct GenFuture<T: Generator<ResumeTy, Yield = ()>>(T);

        // 依赖这样一个重要事实：
        // 即 async/await Future 是不可移动的，以便在基础生成器中创建自引用借用。
        impl<T: Generator<ResumeTy, Yield = ()>> !Unpin for GenFuture<T> {}

        impl<T: Generator<ResumeTy, Yield = ()>> Future for GenFuture<T> {
            type Output = T::Return;
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                // SAFETY: Safe because we're !Unpin + !Drop, and this is just a field projection.
                let gen = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) };

                // Resume the generator, turning the `&mut Context` into a `NonNull` raw pointer. The
                // `.await` lowering will safely cast that back to a `&mut Context`.
                // resume 里传递上下文指针
                match gen.resume(ResumeTy(NonNull::from(cx).cast::<Context<'static>>())) {
                    GeneratorState::Yielded(()) => Poll::Pending,
                    GeneratorState::Complete(x) => Poll::Ready(x),
                }
            }
        }

        GenFuture(gen)
    }

    ```

    Pin 利用类型系统避免 `&mut T` 被拿到，达到 固定 引用/指针 的目的。

    [https://doc.rust-lang.org/std/pin/index.html](https://doc.rust-lang.org/std/pin/index.html)

    - [pin-project-lite](https://crates.io/crates/pin-project-lite)


    ### Pin 与 Unpin 属性

    来自：[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)

    - 如果 `T: Unpin`（默认会实现），那么 `Pin<'a, T>` 完全等价于 `&'a mut T`。换言之： `Unpin` 意味着这个类型被移走也没关系，就算已经被固定了，所以 `Pin` 对这样的类型毫无影响。

    - 如果 `T: !Unpin`， 获取已经被固定的 `T` 类型示例的 `&mut T`需要 `unsafe`。

    - 标准库中的大部分类型实现 `Unpin`，在 Rust 中遇到的多数普通类型也是一样。但是， `async/await` 生成的 `Future` 是个例外。

    - 你可以在 nightly 通过特性标记来给类型添加 `!Unpin` 约束，或者在 stable 给你的类型加 `std::marker::PhatomPinned` 字段。

    - 你可以将数据固定到栈上或堆上

    - 固定 `!Unpin` 对象到栈上需要 `unsafe`

    - 固定 `!Unpin` 对象到堆上不需要` unsafe`。`Box::pin`可以快速完成这种固定。

    - 对于 `T: !Unpin` 的被固定数据，你必须维护好数据内存不会无效的约定，或者叫 固定时起直到释放。这是 固定约定 中的重要部分。


    ### Pin 用法约定

    -  带 Pin 结构化包装的  投影 （`structural Pin projection`） :  `Pin<&mut Wrapper<Field>> -> Pin<&mut Field>`。 `pin_utils::unsafe_pinned!` 宏可以做。
        - 当结构体所有的字段都实现 Unpin ，整个结构体才可以实现 Unpin。（不允许任何实现使用不安全的方式将此类字段移出，比如 `Option::take` 是被禁止的）
        - 结构体不能用 `#[repr(packed)]`
        - 如果`Drop::drop`不移动任何字段，则整个结构只能实现`Drop`
    -  不带 Pin 结构化包装的  投影 (`no structural Pin projection`) :  `Pin<&mut Wrapper<Field>> -> &mut Field`。 ` pin_utils::unsafe_unpinned! ` 宏可以做的。

    参考： [futures-util::future::map](https://github.com/rust-lang/futures-rs/blob/0.3.0-alpha.15/futures-util/src/future/map.rs#L14)

    ```rust
    impl<Fut, F> Map<Fut, F> {
        unsafe_pinned!(future: Fut); // pin projection -----+
        unsafe_unpinned!(f: Option<F>); // not pinned --+   |
    //                                                  |   |
    //                 ...                              |   |
    //                                                  |   |
        fn poll (mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
            //                                          |   |
            match self.as_mut().future().poll(cx) { // <----+ required here
                Poll::Pending => Poll::Pending, //      |
                Poll::Ready(output) => { //             |
                    let f = self.f().take() // <--------+ allows this

    ```

*/
pub fn f_pin_unpin() {}

/**

    # `no-std` 下的异步

    - [futures-micro](https://github.com/irrustible/futures-micro)
    - [embassy](https://github.com/embassy-rs/embassy)
    - [executor for no-std](https://github.com/richardanaya/executor)

*/
pub fn no_std_async() {}

/**
    # 一个异步缓存代码实现源码导读 ：[retainer](https://github.com/ChaosStudyGroup/retainer)

*/
pub fn async_cache_demo() {}
