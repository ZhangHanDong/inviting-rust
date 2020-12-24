//! 第二章：Rust核心概念
//! 2.3 Thread Safe

/**
    ### 理解本地线程，理解并发

    - 并发：同时「应对」很多事的能力
    - 并行：同时「执行」很多事的能力

    [https://doc.rust-lang.org/std/time/struct.Duration.html](https://doc.rust-lang.org/std/time/struct.Duration.html)

    ```
    use std::thread;

    fn main() {
        // Duration 实现了 Copy、Send、Sync
        let duration = std::time::Duration::from_millis(3000);

        println!("Main thread");

        let handle  = thread::spawn(move || {
            println!("Sub thread 1");

            // 注意：它的父线程是主线程，而不是线程1
            let handle2 = thread::spawn( move || {
                println!("Sub thread 2");
                thread::sleep(duration);
            });

            handle2.join().unwrap();
            thread::sleep(duration);
        });

        handle.join().unwrap();
        thread::sleep(duration);
    }
*/
pub fn understand_local_thread(){ 
    println!(" 理解本地线程 ");
}


/**
    ### 线程间共享数据

    [https://doc.rust-lang.org/std/time/struct.Duration.html](https://doc.rust-lang.org/std/time/struct.Duration.html)

    ```
    use std::thread;

    fn main() {
        let mut v = vec![1,2,3];
        thread::spawn(move || {
            v.push(4);
        });
        // Can no longer access `v` here.
    }
    ```

    ```
    // invalid
    use std::thread;

    fn main() {
        let mut v = vec![1,2,3];
        for i in 0..10 {
            thread::spawn(move || {
                v.push(i);
            });
        }
    }
    ```

    借用检查阻止并发Bug

    ```
    // invalid 
    fn inner_func(vref: &mut Vec<u32>) {
        std::thread::spawn(move || {
        vref.push(3);
        });
    }

    fn main() {
        let mut v = vec![1,2,3];
        inner_func(&mut v);
    }
    ```
*/
pub fn understand_shared_thread(){ 
    println!(" 线程间共享数据 ");
}

/**
    ### 使用 Arc 和 Mutex 安全共享数据

    ```
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn main() {
        let v = Arc::new(Mutex::new(vec![1,2,3]));

        for i in 0..3 {
            let cloned_v = v.clone();
            thread::spawn(move || {
                cloned_v.lock().unwrap().push(i);
            });
        }
    }
    ```
*/
pub fn understand_safed_shared_thread(){ 
    println!(" 线程间安全共享数据 ");
}


/**
    ### 构建「无悔」并发系统

    使用 channel 和 condvar ： 模拟并行组件

    - [parking_lot](https://github.com/Amanieu/parking_lot)
    - [crossbeam](https://github.com/crossbeam-rs/crossbeam)

    > 1. Rust 保证安全性上「无畏」，但不保证工程性上的「无悔」。
    > 2. 但 Rust 有提供帮助我们建立「无悔」并发的「工具」。
    > 3. 通过这些工具，结合从实际沉淀出来并发模型的最佳默认模式「event-loop」来建立健壮的并发应用。
    > 4. 拓展阅读：
    > [Rust concurrency patterns: regret-less concurrency](https://medium.com/@polyglot_factotum/rust-regret-less-concurrency-2238b9e53333)


    示例1: 用 channel 模拟 event 

    ```
    #[macro_use]
    extern crate crossbeam_channel;
    extern crate rayon;

    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    use std::sync::{Arc, Condvar, Mutex};
    // use parking_lot::{Mutex, Condvar};
    // use std::sync::Arc;
    use std::thread;

    // 此消息用于发送到与「主组件」并行运行的其他组件。
    enum WorkMsg {
        Work(u8),
        Exit,
    }

    // 此消息用于从并行运行的其他组件 发送回「主组件」。
    enum ResultMsg {
        Result(u8),
        Exited,
    }

    fn main() {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();

        // 生成子线程用于执行另一个并行组件
        let _ = thread::spawn(move || loop {
            // 接收并处理消息，直到收到 exit 消息
            match work_receiver.recv() {
                Ok(WorkMsg::Work(num)) => {
                    // 执行一些工作，并且发送消息给 Result 队列
                    let _ = result_sender.send(ResultMsg::Result(num));
                }
                Ok(WorkMsg::Exit) => {
                    // 发送 exit 确认消息
                    let _ = result_sender.send(ResultMsg::Exited);
                    break;
                }
                _ => panic!("Error receiving a WorkMsg."),
            }
        });

        let _ = work_sender.send(WorkMsg::Work(0));
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Exit);

        // worker执行计数
        let mut counter = 0;

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(num)) => {
                    // 断言确保接收和发送的顺序是一致的
                    assert_eq!(num, counter);
                    counter += 1;
                }
                Ok(ResultMsg::Exited) => {
                    // 断言确保在接收两条工作消息之后收到退出消息
                    assert_eq!(2, counter);
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }
    ```

    示例二：

    ```rust
    #[macro_use]
    extern crate crossbeam_channel;
    extern crate rayon;

    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    // use std::sync::{Arc, Condvar, Mutex};

    use parking_lot::{Condvar, Mutex};
    use std::sync::Arc;
    use std::thread;

    enum WorkMsg {
        Work(u8),
        Exit,
    }

    enum ResultMsg {
        Result(u8),
        Exited,
    }

    fn main() {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();
        // 引入线程池，开两个工作线程
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();

        let _ = thread::spawn(move || loop {
            match work_receiver.recv() {
                Ok(WorkMsg::Work(num)) => {
                    let result_sender = result_sender.clone();
                    // 使用线程池中的线程
                    pool.spawn(move || {
                        // 执行一些工作，并且发送消息给 Result 队列
                        let _ = result_sender.send(ResultMsg::Result(num));
                    });
                }
                Ok(WorkMsg::Exit) => {
                    let _ = result_sender.send(ResultMsg::Exited);
                    break;
                }
                _ => panic!("Error receiving a WorkMsg."),
            }
        });

        let _ = work_sender.send(WorkMsg::Work(0));
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Exit);

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(_)) => {
                    // 不能再断言顺序了
                }
                Ok(ResultMsg::Exited) => {
                    // 也不能断言在退出消息之前已经收到了结果
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }
    ```

    示例3: 确保工作结束再退出

    ```rust
    #[macro_use]
    extern crate crossbeam_channel;
    extern crate rayon;

    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    // use std::sync::{Arc, Condvar, Mutex};

    use parking_lot::{Condvar, Mutex};
    use std::sync::Arc;
    use std::thread;

    enum WorkMsg {
        Work(u8),
        Exit,
    }

    enum ResultMsg {
        Result(u8),
        Exited,
    }

    fn main() {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();
        // 添加一个新的Channel，Worker使用它来通知“并行”组件已经完成了一个工作单元
        let (pool_result_sender, pool_result_receiver) = unbounded();
        let mut ongoing_work = 0;
        let mut exiting = false;
        // 使用线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();

        let _ = thread::spawn(move || loop {
            // 使用 corssbeam 提供的 select! 宏 选择一个就绪工作
            select! {
                recv(work_receiver) -> msg => {
                    match msg {
                        Ok(WorkMsg::Work(num)) => {
                            let result_sender = result_sender.clone();
                            let pool_result_sender = pool_result_sender.clone();

                            // 注意，这里正在池上启动一个新的工作单元。
                            ongoing_work += 1;

                            pool.spawn(move || {
                                // 1. 发送结果给「主组件」
                                let _ = result_sender.send(ResultMsg::Result(num));

                                // 2. 让并行组件知道这里完成了一个工作单元
                                let _ = pool_result_sender.send(());
                            });
                        },
                        Ok(WorkMsg::Exit) => {
                            // N注意，这里接收请求并退出
                            exiting = true;

                            // 如果没有正则进行的工作则立即退出
                            if ongoing_work == 0 {
                                let _ = result_sender.send(ResultMsg::Exited);
                                break;
                            }
                        },
                        _ => panic!("Error receiving a WorkMsg."),
                    }
                },
                recv(pool_result_receiver) -> _ => {
                    if ongoing_work == 0 {
                        panic!("Received an unexpected pool result.");
                    }

                    // 注意，一个工作单元已经被完成
                    ongoing_work -=1;

                    // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                    if ongoing_work == 0 && exiting {
                        let _ = result_sender.send(ResultMsg::Exited);
                        break;
                    }
                },
            }
        });

        let _ = work_sender.send(WorkMsg::Work(0));
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Exit);

        let mut counter = 0;

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(_)) => {
                    // 计数当前完成的工作单元
                    counter += 1;
                }
                Ok(ResultMsg::Exited) => {
                    // 断言检测：是在接收到两个请求以后退出的
                    assert_eq!(2, counter);
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }
    ```

    示例3 重构

    ```rust
    #[macro_use]
    extern crate crossbeam_channel;
    extern crate rayon;

    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    // use std::sync::{Arc, Condvar, Mutex};

    use parking_lot::{Condvar, Mutex};
    use std::sync::Arc;
    use std::thread;

    enum WorkMsg {
        Work(u8),
        Exit,
    }

    enum ResultMsg {
        Result(u8),
        Exited,
    }

    struct WorkerState {
        ongoing: i16,
        exiting: bool,
    }

    impl WorkerState {
        fn init() -> Self {
            WorkerState{ ongoing: 0, exiting: false }
        }
        
        fn set_ongoing(&mut self, count: i16) {
            self.ongoing += count;
        }
        
        fn set_exiting(&mut self, exit_state: bool) {
            self.exiting = exit_state;
        }
        
        fn is_exiting(&self) -> bool {
            self.exiting == true
        }
        
        fn is_nomore_work(&self)-> bool {
            self.ongoing == 0
        }
        
    }

    fn main() {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();
        // 添加一个新的Channel，Worker使用它来通知“并行”组件已经完成了一个工作单元
        let (pool_result_sender, pool_result_receiver) = unbounded();
        let mut worker_state = WorkerState::init();
        
        // 使用线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();

        let _ = thread::spawn(move || loop {
            // 使用 corssbeam 提供的 select! 宏 选择一个就绪工作
            select! {
                recv(work_receiver) -> msg => {
                    match msg {
                        Ok(WorkMsg::Work(num)) => {
                            let result_sender = result_sender.clone();
                            let pool_result_sender = pool_result_sender.clone();

                            // 注意，这里正在池上启动一个新的工作单元。
                            worker_state.set_ongoing(1);

                            pool.spawn(move || {
                                // 1. 发送结果给「主组件」
                                result_sender.send(ResultMsg::Result(num));

                                // 2. 让并行组件知道这里完成了一个工作单元
                                pool_result_sender.send(());
                            });
                        },
                        Ok(WorkMsg::Exit) => {
                            // N注意，这里接收请求并退出
                            // exiting = true;
                            worker_state.set_exiting(true);

                            // 如果没有正则进行的工作则立即退出
                            if worker_state.is_nomore_work() {
                                result_sender.send(ResultMsg::Exited);
                                break;
                            }
                        },
                        _ => panic!("Error receiving a WorkMsg."),
                    }
                },
                recv(pool_result_receiver) -> _ => {
                    if worker_state.is_nomore_work() {
                        panic!("Received an unexpected pool result.");
                    }

                    // 注意，一个工作单元已经被完成
                    worker_state.set_ongoing(-1);

                    // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                    if worker_state.is_nomore_work() && worker_state.is_exiting() {
                        result_sender.send(ResultMsg::Exited);
                        break;
                    }
                },
            }
        });

        work_sender.send(WorkMsg::Work(0));
        work_sender.send(WorkMsg::Work(1));
        work_sender.send(WorkMsg::Exit);

        let mut counter = 0;

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(_)) => {
                    // 计数当前完成的工作单元
                    counter += 1;
                }
                Ok(ResultMsg::Exited) => {
                    // 断言检测：是在接收到两个请求以后退出的
                    assert_eq!(2, counter);
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }
    ```

    示例4: 使用缓存

    ```rust
    #[macro_use]
    extern crate crossbeam_channel;
    extern crate rayon;

    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    use std::sync::{Arc, Condvar, Mutex};

    // use parking_lot::{Condvar, Mutex};
    // use std::sync::Arc;
    use std::thread;

    enum WorkMsg {
        Work(u8),
        Exit,
    }

    enum ResultMsg {
        Result(u8, WorkPerformed),
        Exited,
    }

    struct WorkerState {
        ongoing: i16,
        exiting: bool,
    }

    impl WorkerState {
        fn init() -> Self {
            WorkerState{ ongoing: 0, exiting: false }
        }
        
        fn set_ongoing(&mut self, count: i16) {
            self.ongoing += count;
        }
        
        fn set_exiting(&mut self, exit_state: bool) {
            self.exiting = exit_state;
        }
        
        fn is_exiting(&self) -> bool {
            self.exiting == true
        }
        
        fn is_nomore_work(&self)-> bool {
            self.ongoing == 0
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    enum WorkPerformed {
        FromCache,
        New,
    }

    #[derive(Eq, Hash, PartialEq)]
    struct CacheKey(u8);

    fn main() {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();
        // 添加一个新的Channel，Worker使用它来通知“并行”组件已经完成了一个工作单元
        let (pool_result_sender, pool_result_receiver) = unbounded();
        let mut worker_state = WorkerState::init();
        
        // 使用线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
            
        // 缓存 work ，由 池 中的 worker 共享
        let cache: Arc<Mutex<HashMap<CacheKey, u8>>> = Arc::new(Mutex::new(HashMap::new()));

        let _ = thread::spawn(move || loop {
            // 使用 corssbeam 提供的 select! 宏 选择一个就绪工作
            select! {
                recv(work_receiver) -> msg => {
                    match msg {
                        Ok(WorkMsg::Work(num)) => {
                            let result_sender = result_sender.clone();
                            let pool_result_sender = pool_result_sender.clone();
                            // 使用缓存
                            let cache = cache.clone();

                            // 注意，这里正在池上启动一个新的工作单元。
                            worker_state.set_ongoing(1);

                            pool.spawn(move || {
                                let num = {
                                    // 缓存开始
                                    let cache = cache.lock().unwrap();
                                    let key = CacheKey(num);
                                    if let Some(result) = cache.get(&key) {
                                        // 从缓存中获得一个结果，并将其发送回去，
                                        // 同时带有一个标志，表明是从缓存中获得了它
                                        let _ = result_sender.send(ResultMsg::Result(result.clone(), WorkPerformed::FromCache));
                                        let _ = pool_result_sender.send(());
                                        return;
                                    }
                                    key.0
                                    // 缓存结束
                                };

                                // work work work work work work...

                                // 返回结果，表明我们必须执行work
                                let _ = result_sender.send(ResultMsg::Result(num.clone(), WorkPerformed::New));

                                // 在缓存中存储“昂贵”的work.
                                let mut cache = cache.lock().unwrap();
                                let key = CacheKey(num.clone());
                                cache.insert(key, num);

                                let _ = pool_result_sender.send(());
                            });
                        },
                        Ok(WorkMsg::Exit) => {
                            // N注意，这里接收请求并退出
                            // exiting = true;
                            worker_state.set_exiting(true);

                            // 如果没有正则进行的工作则立即退出
                            if worker_state.is_nomore_work() {
                                result_sender.send(ResultMsg::Exited);
                                break;
                            }
                        },
                        _ => panic!("Error receiving a WorkMsg."),
                    }
                },
                recv(pool_result_receiver) -> _ => {
                    if worker_state.is_nomore_work() {
                        panic!("Received an unexpected pool result.");
                    }

                    // 注意，一个工作单元已经被完成
                    worker_state.set_ongoing(-1);

                    // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                    if worker_state.is_nomore_work() && worker_state.is_exiting() {
                        result_sender.send(ResultMsg::Exited);
                        break;
                    }
                },
            }
        });

        let _ = work_sender.send(WorkMsg::Work(0));
        // 发送两个相同的work
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Work(1));
        let _ = work_sender.send(WorkMsg::Exit);

        let mut counter = 0;

        loop {
            match result_receiver.recv() {
                Ok(ResultMsg::Result(_, _cached)) => {
                    // 计数当前完成的工作单元
                    counter += 1;
                }
                Ok(ResultMsg::Exited) => {
                    // 断言检测：是在接收到两个请求以后退出的
                    assert_eq!(3, counter);
                    break;
                }
                _ => panic!("Error receiving a ResultMsg."),
            }
        }
    }
    ```
*/
pub fn understand_channel_and_condvar(){ 
    println!(" 线程间安全共享数据 ");
}