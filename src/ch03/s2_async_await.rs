//! 第三章：Rust 异步编程概念
//!
//! # 3.2 异步 编程 模型
//! 
//! - Future 
//! - 生成器 与 协程
//! - Pin 与 UnPin
//! - Async/Await
//! 
//! 

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

pub fn futures(){}




/**

    # 生成器 与 协程

    ## 历史

    处理异步事件的三种方式：
    - Callback
    - Promise
    - async/await 

    async/await 是目前体验最好的方式，Rust 要支持它并不容易。

    ## 生成器

    

*/
pub fn generator(){}