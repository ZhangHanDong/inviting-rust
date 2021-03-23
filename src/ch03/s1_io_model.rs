//! 第三章：Rust 异步编程概念
//!
//! # 3.1 异步 I/O 模型
//! 
//! - 基本概念： 同步/异步、阻塞/非阻塞IO、多路复用、epoll/io_uring
//! - Reactor/Preactor模型 与 事件抽象
//! - minimio/mio


/**

    # 异步 I/O 模型

    ## 基本概念

    - 同步和异步，关注的是消息通信机制。（调用者视角）
        - 同步，发出一个调用，在没有得到结果之前不返回。
        - 异步，发出一个调用，在没有得到结果之前返回。
    - 阻塞和非阻塞，关注的是程序等待调用结果的状态。（被调用者视角）
        - 阻塞，在调用结果返回之前，线程被挂起。
        - 非阻塞，在调用结果返回之前，线程不会被挂起。
    
    阻塞，与系统调用有关。


    ### I/O 模型

    ```text
                                     +-+ 阻 塞 I/O (BIO)
                                     |
                                     +-+ 非 阻 塞 I/O (NIO)
                                     |
                  +----+ 同 步 I/O +--+
                  |                  |
                  |                  +-+ I/O 多 路 复 用
                  |                  |
                  |                  +-+ 信 号 驱 动 I/O
    I/O 模 型  +---+
                  |
                  |
                  |                  +-+ Linux (AIO)
                  |                  |         (io_uring)
                  +----+ 异 步 I/O +--+
                                     |
                                     +-+ windows (IOCP)

    ```

    ### 同步阻塞I/O (blocking I/O)

    ```text
    Application               kernel
    +---------+            +-----------+  +---+
    |         |   syscall  | no        |      |
    |   Read  | +--------> | datagram  |      |
    | recvfrom|            | ready     |      |
    |         |            |    +      |      +-+ wait for
    |         |            |    |      |      +-+ data
    |         |            |    v      |      |
    |         |            | datagram  |      |
    |         |            | ready     |  +---+
    |         |            |           |
    |         |            | copy      |  +---+
    |         |            | datagram  |      |
    |process  |            |    +      |      +-+ copy data
    |datagram |   return   |    |      |      +-+ from kernel to user
    |         | <--------+ |    v      |      |
    |         |            |  copy     |  +---+
    |         |            |  complete |
    +---------+            +-----------+
    ```

    输入操作两个阶段：

    1. 进程等待内核把数据准备好；这个阶段可以阻塞也可非阻塞，设置socket属性。
        - 阻塞： recvfrom 阻塞线程直到返回数据就绪的结果。
        - 非阻塞：立即返回一个错误，轮询直到数据就绪。
    2. 从内核缓冲区向进程缓冲区复制数据。（一直阻塞）

    异步I/O，recvfrom总是立即返回，两个阶段都由内核完成。

    ### I/O 多路复用（I/O Multiplexing )

    IO多路复用是一种同步IO模型，实现一个线程可以监视多个文件句柄。

    支持I/O多路复用的系统调用有 select/pselect/poll/epoll，本质都是 同步 I/O，因为数据拷贝都是阻塞的。
    通过 select/epoll 来判断数据报是否准备好，即判断可读可写状态。


    

 */
pub fn basic_concept(){}



/**

    ## epoll

    ```text
                            +--------------------------------+     +-------------------------+
                            | epoll_ctl                      |     | epoll_wait              |
                            |                                |     |                         |
                            |                                |     |         +----+          |
                            |                 +---+          |     |         |    |          |
                            |                 |   |          |     |         |    |          |
                            |               +-+---+--+       |     |         +--+-+          |
                            |               |        |       |     |            |            |
                            |            +--++     +-++      |     |            |            |
    epoll_create  +---->    |            |   |     |  |      |     |         +--+-+          |
                            |            +-+-+     +--+      +---->+         |    |          |
                            |              |                 |event|         |    |          |
                            |         +----+--+              |     |         +--+-+          |
                            |         |       |              |     |            |            |
                            |         ++      |              |     |            |            |
                            |        +--+   +-+-+            |     |         +--+-+          |
                            |        |  |   |   |            |     |         |    |          |
                            |        +--+   +---+            |     |         |    |          |
                            |                                |     |         +----+          |
                            |                    红 黑 树     |     |                 链 表    |
                            +--------------------------------+     +-------------------------+


    ```

    - epoll_create(int size) : 内核产生一个epoll实例数据结构，并返回一个epfd
    - epoll_ctl(int epfd, int op, int fd, struct epoll_event *event)：将被监听的描述符添加到红黑树或从红黑树中删除或者对监听事件进行修改。
    - epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout): 阻塞等待注册的事件发生，返回事件的数目，并将触发的事件写入events数组中


    epoll 两种触发机制：

    - 水平触发机制（LT)。缓冲区只要有数据就触发读写。epoll 默认工作方式。select/poll只支持该方式。
    - 边缘触发机制（ET)。缓冲区空或满的状态才触发读写。nginx 使用该方式，避免频繁读写。

    惊群问题：

    当多个进程/线程调用epoll_wait时会阻塞等待，当内核触发可读写事件，所有进程/线程都会进行响应，但是实际上只有一个进程/线程真实处理这些事件。
    Liux 4.5 通过引入 EPOLLEXCLUSIVE 标识来保证一个事件发生时候只有一个线程会被唤醒，以避免多侦听下的惊群问题。
*/
pub fn epoll(){}


/**
    ## io_uring 异步 I/O 模型

    Linux AIO 实现的并不理想，所以引入了新的异步I/O接口 io_uring。

    ```text
    +----+ Head  +---------+               +----------+ Head
    |            |         |               |          |
    |            |         |               |          |
    |            +---------+               +----------+
    |            |         |               |          |
    |            |         |               |          |
    |            +---------+               +----------+
    |            |         |               |          |
    |            |         |               |          |
    |            +---------+               +----------+
    |            |         |               |          |
    |      Tail  +---------+               +----------+ Tail <--+
    |        +--------------------------------------------+     |
    |        | Kernel                                     |     |
    |        |                                            |     |
    |        |        +-------+              +-------+    |     |
    |        |        |       |              |       |    |     |
    +---------------> | SQ    |              |  CQ   | +--------+
             |        |       |              |       |    |
             |        +-------+              +-------+    |
             |                                            |
             +--------------------------------------------+

    ```

    io_uring接口通过两个主要数据结构工作：
    
    - 提交队列条目（sqe）
    - 完成队列条目（cqe）
    
    这些结构的实例位于内核和应用程序之间的**共享内存**单生产者单消费者环形缓冲区中。

    参考：
    
    [https://thenewstack.io/how-io_uring-and-ebpf-will-revolutionize-programming-in-linux/](https://thenewstack.io/how-io_uring-and-ebpf-will-revolutionize-programming-in-linux/)

    [https://cor3ntin.github.io/posts/iouring/#io_uring](https://cor3ntin.github.io/posts/iouring/#io_uring)

*/
pub fn io_uring(){}



/**

    ## 事件驱动编程模型

    因为处理 I/O 复用的编程模型相当复杂，为了简化编程，引入了下面两种模型。

    - Reactor（反应器） 模式，对应同步I/O，被动的事件分离和分发模型。服务等待请求事件的到来，再通过不受间断的同步处理事件，从而做出反应。
    - Preactor（主动器） 模式，对应异步I/O，主动的事件分离和分发模型。这种设计允许多个任务并发的执行，从而提高吞吐量；并可执行耗时长的任务（各个任务间互不影响）。

    Reactor Model: 

    ```text
                                                         +----------------+
    req                                        Dispatch  |                |
    +------+                                  +--------> | req handler    |
    |      |                                  |          +----------------+
    |      | +----+                           |
    +------+      | event    +------------+   |
                  |          |            |   |
                  +--------> |  Service   |   |Dispatch  +----------------+
                             |  Handler   +------------> |                |
    req          +---------> |            |   |          | req handler    |
    +------+     |           +------------+   |          +----------------+
    |      |     | event                      |
    |      +----+                             |
    +------+                                  | Dispatch +----------------+
                                              +--------->+                |
                                                         | req handler    |
                                                         +----------------+

    ```

    三种实现方式：

    - 单线程模式。 accept()、read()、write()以及connect()操作 都在同一线程。
    - 工作者线程池模式。非 I/O 操作交给线程池处理
    - 多线程模式。主Reactor (master) ，负责网络监听 ， 子Reactor(worker) 读写网络数据。

    读写操作流程：

    1. 应用注册读写就绪事件和相关联的事件处理器
    2. 事件分离器等待事件发生
    3. 当发生读写就绪事件，事件分离器调用已注册的事件处理器
    4. 事件处理器执行读写操作

    参与者：
    1. 描述符（handle）：操作系统提供的资源，识别 socket等。
    2. 同步事件多路分离器。开启事件循环，等待事件的发生。封装了 多路复用函数 select/poll/epoll等。
    3. 事件处理器。提供回调函数，用于描述与应用程序相关的某个事件的操作。
    4. 具体的事件处理器。事件处理器接口的具体实现。使用描述符来识别事件和程序提供的服务。
    5. Reactor 管理器。事件处理器的调度核心。分离每个事件，调度事件管理器，调用具体的函数处理某个事件。

*/
pub fn event_driven(){}


/**

    ## Rust 实现 epoll server 示例讲解

    1. [https://github.com/zupzup/rust-epoll-example/blob/main/src/main.rs](https://github.com/zupzup/rust-epoll-example/blob/main/src/main.rs)
    2. [Reactor executor Example](https://github.com/zupzup/rust-reactor-executor-example)

    ## 实现跨平台 

    1. [minimio](https://github.com/cfsamson/examples-minimio)
    2. [mio](https://github.com/tokio-rs/mio) and mio-examples

*/
pub fn epoll_server(){}