// async/await non-blocking echo server

use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::io;
use std::io::Write;
use std::mem;
use std::os::unix::io::RawFd;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};

use lazy_static::lazy_static;
use log::info;

#[macro_use]
mod util;

mod tcp_listener;
mod reactor;
mod epoll;
mod executor;

use libc;

use tcp_listener::{Ipv4Addr, TcpListener,TcpStream};
use reactor::{Reactor, reactor_main_loop};
use epoll::{Epoll, EpollEventType};
use executor::{new_executor_and_spawner};

lazy_static! {
    static ref REACTOR: Reactor = {
        // Start reactor main loop
        std::thread::spawn(move || {
            reactor_main_loop()
        });

        Reactor {
            epoll: Epoll::new().expect("Failed to create epoll"),
            wakers: Mutex::new(HashMap::new())
        }
    };
}


fn init_log() {
    // format = [file:line] msg
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{:>3}] {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args(),
            )
        })
        .init();
}

// async fn f() -> T
// equivalent to ：
// fn f() -> impl Future<Output = T>

async fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    info!("(handle client) {}", stream.0);
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        stream.write(&buf[..n]).await?;
    }
    Ok(())
}


fn main() {
    init_log();

    let (executor, spawner) = new_executor_and_spawner();
    let spawner_clone = spawner.clone();

    let mainloop = async move {
        let addr = Ipv4Addr::new(127, 0, 0, 1);
        let port = 8080;
        let listner = TcpListener::bind(addr, port)?;

        let incoming = listner.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            spawner.spawn(handle_client(stream));
        }

        Ok(())
    };

    spawner_clone.spawn(mainloop);
    drop(spawner_clone);
    executor.run();
}
