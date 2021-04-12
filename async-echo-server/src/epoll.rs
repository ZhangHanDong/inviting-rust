use super::*;
use libc;

pub struct Epoll {
    fd: RawFd,
}

pub enum EpollEventType {
    // Only event types used in this example
    In,
    Out,
}

impl Epoll {
    // 需要 unix 平台
    pub fn new() -> io::Result<Self> {
        let fd = syscall!(epoll_create1(libc::EPOLL_CLOEXEC))?;
        Ok(Epoll { fd })
    }

    fn run_ctl(&self, epoll_ctl: libc::c_int, fd: RawFd, op: EpollEventType) -> io::Result<()> {
        let mut event: libc::epoll_event = unsafe { mem::zeroed() };
        event.u64 = fd as u64;
        event.events = match op {
            EpollEventType::In => libc::EPOLLIN as u32,
            EpollEventType::Out => libc::EPOLLOUT as u32,
        };

        let eventp: *mut _ = &mut event as *mut _;
        syscall!(epoll_ctl(self.fd, epoll_ctl, fd, eventp))?;

        Ok(())
    }

    pub fn add_event(&self, fd: RawFd, op: EpollEventType) -> io::Result<()> {
        self.run_ctl(libc::EPOLL_CTL_ADD, fd, op)
    }

    #[allow(dead_code)]
    pub fn mod_event(&self, fd: RawFd, op: EpollEventType) -> io::Result<()> {
        self.run_ctl(libc::EPOLL_CTL_MOD, fd, op)
    }

    pub fn del_event(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(
            self.fd,
            libc::EPOLL_CTL_DEL,
            fd,
            std::ptr::null_mut() as *mut libc::epoll_event
        ))?;

        Ok(())
    }

    pub fn wait(&self, events: &mut [libc::epoll_event]) -> io::Result<usize> {
        let nfd = syscall!(epoll_wait(
            self.fd,
            events.as_mut_ptr(),
            events.len() as i32,
            -1
        ))?;

        Ok(nfd as usize)
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        syscall!(close(self.fd)).ok();
    }
}
