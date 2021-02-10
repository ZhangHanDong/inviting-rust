//! 第二章：Rust核心概念
//! 2.3 Lockfree
//! 


/**
 
    ### 并发编程注重的三点：

    1. 原子性。保证操作是原子的。
    2. 可见性。保证数据是同步的。
    3. 顺序性。保证操作的顺序是正确的。

    方法：

    - 同步锁
    - 无锁编程


    ### 思考：锁带来的问题？

    1. 性能。引入无锁编程可以最大化减少线程上下文切换，线程等待。
    2. 死锁。引入无锁编程就不会产生死锁。

    无锁编程性能并不是总是优于锁同步。

    无锁编程依赖于原子类型，使用原子类型还需要深入了解一些概念。


    ###  理解无锁并发的关键在于理解计算机组成
 
    ```text
    +------+    +------+    +------+
    | core |    | core |    | core |
    +---+--+    +---+--+    +---+--+
        |           |           |
    +lv1+--+    +-lv1--+    +lv1---+
    |cache |    |cache |    | cache|
    +---+--+    +---+--+    +---+--+
        |           |           |
    +lv2---+    +lv2---+    +lv2---+
    | cache|    | cache|    | cache|
    +---+--+    +---+--+    +---+--+
        |           |           |
        +-----------------------+
                    |
    +---------------+--------------+
    |           lv3 cache          |
    +---------------+--------------+
                    |
                    |
    +---------------+--------------+
    |          main memory         |
    +------------------------------+

    ```

    ### 缓存一致性问题

    思考：下面代码最终执行后 x 和 y 的状态上什么？

    ``` text
    // THREAD 1                       

    if unsafe { *x == 1 } {
        unsafe { *y += 1 }
    }                   
    
    // THREAD 2
    unsafe {
        *y = 10;
        *x = 1;
    }
    ```

    最终结果实际取决于下面几个问题：

    1. THREAD 1 和 THREAD 2 运行顺序是什么？ （使用锁同步可以保证“喂给” CPU 的指令锁有顺序性的）
    2. 每一个 CPU 使用的高速缓存的状态 （要保证缓存一致性）
    3. CPU 指令重排（乱序执行，为了更好的利用流水线，达到性能极致）
    4. 编译器指令重排 （调整指令顺序，使其对 CPU 更友好）


    > 高速缓存通过 M(Modified)E(Exclusive)S(Shared)I(Invalid) 协议来保持同步。MESI的有趣之处在于，每个高速缓存行都在维护自己的内存地址状态机。
    > 一个 CPU 通过 IPC (Inter-Processor-Communication) 向 另一个 CPU 来发送消息，比如，「独占内存地址」、「修改内存数据」等。
    > 这里要注意，CPU 并不是做来某些操作马上发出消息或者是收到消息马上就执行相应操作的。也就是说，CPU 通信的这些消息都是异步的。
    > 如果 CPU 多核之间用同步通信的话，性能上无法接受。比如一个 CPU 要等待其他 CPU 来确认信息，或从其他 CPU 获取最新数据等。
    > 所以，为了让 CPU 核之间可以高性能地同步信息（保证cpu乱序执行指令的同时，还要保证程序正确性），就引入了内存屏障的技术。
    

    ### 内存屏障
    
    内存屏障允许开发者在编写代码等时候在需要等地方加入它。

    内存屏障分为：

    1. 读屏障（Load Memory Barrier）。任何「读屏障」前的「读操作」都会先于「读屏障」后的「读操作」完成。
    2. 写屏障（Store Memory Barrier）。任何「写屏障」前的「写操作」都会先于「写屏障」后的「写操作」完成。
    3. 全屏障。同时包含读屏障和写屏障的作用。

    load，代表从内存读数据。store，代表向内存写数据。

    现代 CPU 架构中一般分为四种内存屏障：

    1. Load-Load: 等价于上面介绍的「读屏障」。任何「该屏障」前的「读操作」都会先于「该屏障」后的「读操作」完成。
    2. Load-Store: 任何「该屏障」前的「读操作」都会先于「该屏障后」的「写操作」完成。
    3. Store-Load: 任何「该屏障」前的「写操作」都会先于「该屏障后」的「读操作」完成。
    4. Store-Store: 等价于上面介绍的「写屏障」。任何「该屏障」前的「写操作」都会先于「该屏障」后的「写操作」完成。


    开发者通过内存屏障，告诉 CPU/编译器 内存屏障前后指令的顺序关系，这样 CPU/编译器 就不会重排这些指令，从而保证原子指令的顺序性。

    ### 内存模型

    这么多内存屏障，什么时候该使用哪种，需要由开发者来指定。这道理和 Unsafe Rust 类似，编译器无法检查的安全性，交给开发者。

    这就是语言提供内存模型的原因。Cpp 和 Rust 语言都提供了原子（Atomic）类型，并且这些原子类型都是可以指定内存顺序（告诉CPU使用哪种内存屏障）

    Rust 目前并没有正式的内存顺序模型，但是它在语义和行为上和 Cpp 一致。

    由此引申出内存屏障都两种语义：

    1. acquire（获取）语义。Load 之后的读写操作无法被重排至 Load 之前。即 相当于Load-Load和Load-Store屏障。
    2. release（释放）语义。Store 之前的读写操作无法被重排至 Store 之后。即 相当于Load-Store和Store-Store屏障。


    ### 原子操作

    Rust 标准库中定义的原子类型：[std::sync::atomic: https://doc.rust-lang.org/stable/std/sync/atomic/index.html](https://doc.rust-lang.org/stable/std/sync/atomic/index.html)
    其中`// std::sync::atomic::Ordering`定义了 Rust 支持的内存顺序，官方文档指出，当前和 Cpp20 的内存顺序是一致的。
    

    ```rust
    // std::sync::atomic::Ordering

    pub enum Ordering {
        Relaxed,
        Release,
        Acquire,
        AcqRel,
        SeqCst,
    }
    ```

    注意，Rust 当前并没有公开 cpp 里面包含对 consume 语义。

    内存顺序说明：

    - Relaxed，表示原子类型只保证原子操作即可，没有内存顺序（不指定内存屏障）
    - Release，
        - 表示使用 Release 语义。
        - 当前线程内的所有写操作，对于其他对这个原子变量进行 acquire 的线程可见
    - Acquire，
        - 表示使用 Acquire 语义。
        - acquire 可以保证读到所有在 release 前发生的写入。
    - AcqRel，
        - 表示对读取和写入施加 acquire-release 语义，无法被重排。
        - 可以看见其他线程施加 release 语义的所有写入，同时自己的 release 结束后所有写入对其他施加 acquire 语义的线程可见。
    - SeqCst，
        - 如果是读取就是 acquire 语义，如果是写入就是 release 语义，如果是读取+写入就是 acquire-release 语义。
        - 所有线程都能以相同的顺序看到所有顺序一致的操作。
    
    不同对内存顺序，对应不同对内存屏障，进一步，也代表了不同的性能。
    在竞争条件比较激烈的情况下，Relaxed 性能是最好的，因为它不需要任何内存屏障，这就意味着CPU之间不需要进行一致性同步。
    相对而言，SeqCst 就是性能最差的那个了，因为它需要 CPU 同步所有指令。
    但是 Relaxed 因为没有内存屏障，所以可能会有指令重排带来带风险。
    所以，Rust 标准库也提供了` std::sync::atomic::compiler_fence`和` std::sync::atomic::fence`两个函数，来帮助解决在原子指令使用 Relaxed 内存顺序的情况下编译器或CPU指令重排的问题。

    示例：[https://doc.rust-lang.org/stable/std/sync/atomic/fn.compiler_fence.html](https://doc.rust-lang.org/stable/std/sync/atomic/fn.compiler_fence.html)

    原子类型提供的方法，基于硬件原子指令 (x均为std::atomic)	：

    - `load`，返回x的值。
    - `store`，把x设为n，什么都不返回。
    - `swap`，把x设为n，返回设定之前的值。
    - `compare_and_swap`，经典cas操作。
    - `compare_exchange.
    - `compare_exchange_weak
    - `fetch_add(n), fetch_sub(n)`，原子地做x += n, x-= n，返回修改之前的值。

   
    使用原子类型需要注意的是：

    - Store操作，可选内存顺序：Relaxed, Release, SeqCst。否则panic。
    - Load操作，可选内存顺序：Relaxed, Acquire, SeqCst。否则panic。
    - Read-modify-write(读-改-写)操作，可选如下顺序：Relaxed, Acquire, Release, AcqRel, SeqCst。
    - 所有操作的默认顺序都是 SeqCst。

     ```rust
    // 实现一个简单的自旋锁（spinlock）
    
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::{thread, time};

    fn main() {
        let spinlock = Arc::new(AtomicUsize::new(1));

        let spinlock_clone = Arc::clone(&spinlock);
        let thread = thread::spawn(move|| {
            // lock
            spinlock_clone.store(1, Ordering::SeqCst);
            // do something
            let t = time::Duration::from_secs(3);
            std::thread::sleep(t);
            // unlock
            spinlock_clone.store(0, Ordering::SeqCst);
        });

        // Wait for the other thread to release the lock
        while spinlock.load(Ordering::SeqCst) != 0 {}

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {:?}", panic);
        }
    }

    ```

    利用 AtomicBool 实现一个 轻量级的锁 ：

    ```rust
    use std::sync::atomic::Ordering;
    use core::sync::atomic::AtomicBool;

    struct LightLock(AtomicBool);

    impl LightLock {
        pub fn new() -> LightLock {
            LightLock(AtomicBool::new(false))
        }

        pub fn try_lock<'a>(&'a self) -> Option<LightGuard<'a>> {
            let was_locked = self.0.swap(true, Ordering::Acquire);
            if was_locked {
                None
            } else {
                Some(LightGuard { lock: self })
            }
        }
    }

    struct LightGuard<'a> {
        lock: &'a LightLock,
    }

    impl<'a> Drop for LightGuard<'a> {
        fn drop(&mut self) {
            self.lock.0.store(false, Ordering::Release);
        }
    }
    ```

    ### ABA 问题：

    任何无 GC 的语言在无锁编程的时候都要考虑此问题。

    试想一个连续压栈（push）和 出栈（pop）的并发操作。假设这两个操作都是由 cas 原语实现的。

    > 具体来说，假如有两个线程 T1 和 T2，操作初始栈为「a->b->c」的栈结构。
    > 当 T1 把 a 从栈内弹出，此时发生线程调度，
    > 切换到 T2 ，T2 弹出 a 和 b，把 a 再 push 到栈里，此时 T2 的栈为 「a->c」。
    > 然后线程切回 T1 ，T1 看到栈顶（a）的地址和它之前获得的 a 地址相同，然后将 栈顶设置为 b （a.next），然而 b 早就被释放来。

    这就是 ABA 问题。ABA 问题本质是内存回收问题。当 b 被弹出当时候，要保障它当内存不能被立即重用。
    
    解决该问题的思路有多种：引用计数、分代回收（Epoch Based Reclamation）和 险象指针（Hazard pointer）。

    注意：ABA 问题一般是发生在 X86 架构上 cas 原子操作的时候。ARM 架构已经从根源上解决了 ABA 问题。

    - 分代回收示例：[https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-epoch/examples/sanitize.rs](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-epoch/examples/sanitize.rs)
    - 险象指针示例：
        - [https://github.com/solotzg/rs-lockfree/blob/master/src/hazard_pointer.rs](https://github.com/solotzg/rs-lockfree/blob/master/src/hazard_pointer.rs)
        -  [https://github.com/redox-os/conc/blob/master/src/sync/treiber.rs](https://github.com/redox-os/conc/blob/master/src/sync/treiber.rs)



*/
pub fn memory_reordering(){}