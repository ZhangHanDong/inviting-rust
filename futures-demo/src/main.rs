use futures::executor::{ThreadPool,block_on,ThreadPoolBuilder};
use futures::future::{ Future};
use futures::task::{Context, Poll,};
use std::thread;

use std::pin::Pin;

fn main(){
    const NUM: usize = 20;

    struct Yield {
        rem: usize,
    }

    impl Future for Yield {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.rem == 0 {
                println!("{}:done",thread::current().name().unwrap());
                Poll::Ready(())
            } else {
                println!("self.rem={}",self.rem);
                // note thread safe 
                self.rem -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
   
    // for _ in 0..NUM {
    //     let y = Yield { rem: NUM };
    //     block_on(y) 
    // }


    // 类似 epoll 惊群


    
    for i in 0..NUM {
        let y = Yield { rem: NUM };
        let name_prefix = format!("pool-");
        let pool = ThreadPoolBuilder::new().pool_size(8).name_prefix(name_prefix).create().unwrap();
        pool.spawn_ok(y);   
    }
}
