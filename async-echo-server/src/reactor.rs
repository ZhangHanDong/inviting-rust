use super::*;

pub struct Reactor {
    pub epoll: Epoll,
    pub wakers: Mutex<HashMap<RawFd, Waker>>,
}


impl Reactor {
    pub fn add_event(&self, fd: RawFd, op: EpollEventType, waker: Waker) -> io::Result<()> {
        info!("(Reactor) add event: {}", fd);
        self.epoll.add_event(fd, op)?;
        self.wakers.lock().unwrap().insert(fd, waker);
        Ok(())
    }
}

pub fn reactor_main_loop() -> io::Result<()> {
    info!("Start reactor main loop");
    let max_event = 32;
    let event: libc::epoll_event = unsafe { mem::zeroed() };
    let mut events = vec![event; max_event];
    let reactor = &REACTOR;

    loop {
        let nfd = reactor.epoll.wait(&mut events)?;
        info!("(Reacotr) wake up. nfd = {}", nfd);

        #[allow(clippy::needless_range_loop)]
        for i in 0..nfd {
            let fd = events[i].u64 as RawFd;
            let waker = reactor
                .wakers
                .lock()
                .unwrap()
                .remove(&fd)
                .unwrap_or_else(|| panic!("not found fd {}", fd));
            info!("(Reacotr) delete event: {}", fd);
            reactor.epoll.del_event(fd)?;
            waker.wake();
        }
    }
}