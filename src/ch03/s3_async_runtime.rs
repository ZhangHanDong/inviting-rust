//! 第三章：Rust 异步编程概念
//!
//! # 3.3 异步 运行时
//!
//!
//! - [smol](https://github.com/smol-rs/smol)
//! - [tokio](https://github.com/tokio-rs/tokio)
//! - [async-std](https://github.com/async-rs/async-std)
//! - [bastion](https://github.com/bastion-rs/bastion)
//! - [glommio](https://github.com/DataDog/glommio)
//! - [embassy](https://github.com/embassy-rs/embassy)
//!
//! 框架剖析：
//!
//!
//! - [rocket](https://github.com/SergioBenitez/Rocket)
//! - [acitx-web](https://github.com/actix/actix-web)
//! - [tide](https://github.com/http-rs/tide)
//! - [lunatic](https://github.com/lunatic-solutions/lunatic)
//!
//!

/**

    # 通过学习 smol 建立整体异步编程概念框架

    见 项目 README 图示。soml 的整体架构非常简单和清晰。

    以下是关键组件。

    ## async-task

    对 异步任务 的抽象

    ## async-executor

    异步任务调度和执行，依赖 async-task

    ## async-io

    对接底层I/O，并实现了 Reactor ，依赖 polling

    ## polling

    一层轻薄的 I/O 抽象 API，支持 epoll(Linux/Android)/kqueue(macOS, iOS, FreeBSD, NetBSD, OpenBSD, DragonFly BSD)/event ports(illumos, Solaris)/wepoll(Windows)

    ## blocking

    为异步程序隔离阻塞I/O的线程池。因为像 epoll_wait 实际还是阻塞。


*/
pub fn a_smol_runtime() {}

/**

    # async-std 运行时架构

*/
pub fn b_async_std_runtime() {}

/**

    # Tokio 运行时 架构

    [io_uring 支持](https://github.com/tokio-rs/tokio-uring/pull/1)
*/
pub fn c_tokio_runtime() {}

/**

    # 其他运行时 架构

    - [https://github.com/DataDog/glommio](https://github.com/DataDog/glommio)
    - [https://github.com/bastion-rs/bastion](https://github.com/bastion-rs/bastion)
*/
pub fn d_others_runtime() {}
