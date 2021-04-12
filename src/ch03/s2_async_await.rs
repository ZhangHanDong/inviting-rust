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

    可以说，异步任务的行为是模仿 线程 来抽象。

    1. 线程在进程内，异步任务在线程内。
    2. 线程可被调度切换（Linux默认抢占式），异步任务也可以被调度（协作式而非抢占式）。区别在于，异步任务只在用户态，没有线程的上下文切换开销。
    3. 线程有上下文信息，异步任务也有上下文信息。
    4. 线程间可以通信，异步任务之间也可以通信。
    5. 线程间有竞争，异步任务之间也有竞争。

    整个异步编程概念，包括异步语法、异步运行时都是围绕如何建立这种「绿色线程」抽象而成的。
    
*/
pub fn a_async_intro(){}

/**

    # Future 和 Futures-rs
    
    - [Future](https://doc.rust-lang.org/std/future/index.html) and [task](https://doc.rust-lang.org/std/task/index.html)
    - [futures-rs](https://github.com/rust-lang/futures-rs)

    示例：

    ```rust
    pub struct SocketRead<'a> {
        socket: &'a Socket,
    }

    impl SimpleFuture for SocketRead<'_> {
        type Output = Vec<u8>;

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                // The socket has data-- read it into a buffer and return it.
                Poll::Ready(self.socket.read_buf())
            } else {
                // The socket does not yet have data.
                //
                // Arrange for `wake` to be called once data is available.
                // When data becomes available, `wake` will be called, and the
                // user of this `Future` will know to call `poll` again and
                // receive data.
                self.socket.set_readable_callback(wake);
                Poll::Pending
            }
        }
    }
    ```

*/

pub fn b_futures_simple_intro(){}


/**

    通过实现一个简单的 async/await echo server 来理解 Futures

    代码示例参见本仓库内：async-echo-server
*/

pub fn c_async_await_echo(){}



/**

    深入理解 Future 和 Futures-rs

    - [Future](https://doc.rust-lang.org/std/future/index.html) and [task](https://doc.rust-lang.org/std/task/index.html)
    - [futures-rs](https://github.com/rust-lang/futures-rs)
    - [futures-lite](https://github.com/smol-rs/futures-lite)


*/

pub fn d_futures_rs(){}




/**


    # 异步实现细节：生成器 与 协程

    ## 历史

    处理异步事件的三种方式：
    - Callback
    - Promise
    - async/await 

    async/await 是目前体验最好的方式，Rust 要支持它并不容易。

    ## 生成器


    


*/
pub fn e_generator(){}


/**


    # 异步实现细节：Pin 与 UnPin

    - [pin-project-lite](https://crates.io/crates/pin-project-lite)
    

*/
pub fn f_pin_unpin(){}


/**

    [futures-micro](https://github.com/irrustible/futures-micro)

*/
pub fn no_std_async(){}