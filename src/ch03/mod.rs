//! 第三章：Rust 异步编程概念
//!
//! 本章至下而上的方式来带领大家理解异步编程:
//! 
//! 1. 异步 I/O 模型
//! 2. 异步编程模型：
//!     - 事件驱动模型
//!     - Futures
//!     - 生成器 与 Pin
//!     - async/await
//!     - 异步运行时介绍：async-std、tokio、bastion、smol
//! 

// section 01
pub mod s1_io_model;
// section 02
pub mod s2_async_await;
// section 03
pub mod s3_async_runtime;
