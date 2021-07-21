use super::*;

pub struct Ipv4Addr(libc::in_addr);
pub struct TcpListener(RawFd);
pub struct TcpStream(pub RawFd);
pub struct Incoming<'a>(&'a TcpListener);

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        let in_addr_t = u32::from_be_bytes([a, b, c, d]);
        Ipv4Addr(libc::in_addr {
            s_addr: unsafe { crate::util::htonl(in_addr_t) },
        })
    }
}

pub struct AcceptFuture<'a>(&'a TcpListener);
pub struct ReadFuture<'a>(&'a TcpStream, &'a mut [u8]);
pub struct WriteFuture<'a>(&'a TcpStream, &'a [u8]);


impl TcpListener {
    // NOTE: bind() may be block. So this should be an async function in reality.
    pub fn bind(addr: Ipv4Addr, port: u16) -> io::Result<TcpListener> {
        let backlog = 128;
        let sock = syscall!(socket(
            libc::PF_INET,
            libc::SOCK_STREAM | libc::SOCK_CLOEXEC,
            libc::IPPROTO_IP
        ))?;
        // set server_socket_fd to non-blocking IO
        unsafe {
            let flags = libc::fcntl(sock, libc::F_GETFL, 0);
            libc::fcntl(sock, libc::F_SETFL, libc::O_NONBLOCK | flags);
        }

        let opt: i32 = 1;
        syscall!(setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &opt as *const _ as *const libc::c_void,
            std::mem::size_of_val(&opt) as u32
        ))?;

        let sin: libc::sockaddr_in = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: port.to_be(),
            sin_addr: addr.0,
            ..unsafe { mem::zeroed() }
        };
        let addrp: *const libc::sockaddr = &sin as *const _ as *const _;
        let len = mem::size_of_val(&sin) as libc::socklen_t;

        syscall!(bind(sock, addrp, len))?;
        syscall!(listen(sock, backlog))?;

        info!("(TcpListner) listen: {}", sock);
        let listner = TcpListener(sock);
        Ok(listner)
    }

    pub fn accept(&self) -> io::Result<TcpStream> {
        let mut sin_client: libc::sockaddr_in = unsafe { mem::zeroed() };
        let addrp: *mut libc::sockaddr = &mut sin_client as *mut _ as *mut _;
        let mut len: libc::socklen_t = unsafe { mem::zeroed() };
        let lenp: *mut _ = &mut len as *mut _;
        let sock_client = syscall!(accept(self.0, addrp, lenp))?;
        info!("(TcpStream)  accept: {}", sock_client);
        Ok(TcpStream(sock_client))
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming(self)
    }
}

impl<'a> Incoming<'a> {
    pub fn next(&self) -> AcceptFuture<'a> {
        AcceptFuture(self.0)
    }
}


impl<'a> Future for AcceptFuture<'a> {
    type Output = Option<io::Result<TcpStream>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.accept() {
            Ok(stream) => {
                stream.setnonblocking()?;
                Poll::Ready(Some(Ok(stream)))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                REACTOR.add_event((self.0).0, EpollEventType::In, cx.waker().clone())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl TcpStream {
    pub fn setnonblocking(&self) -> io::Result<()> {
        let flag = syscall!(fcntl(self.0, libc::F_GETFL, 0))?;
        syscall!(fcntl(self.0, libc::F_SETFL, flag | libc::O_NONBLOCK))?;
        Ok(())
    }

    pub fn read<'a>(&'a self, buf: &'a mut [u8]) -> ReadFuture<'a> {
        ReadFuture(self, buf)
    }

    pub fn write<'a>(&'a self, buf: &'a [u8]) -> WriteFuture<'a> {
        WriteFuture(self, buf)
    }
}

impl<'a> Future for ReadFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = syscall!(read(
            (self.0).0,
            self.1.as_mut_ptr() as *mut libc::c_void,
            self.1.len()
        ));
        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                REACTOR.add_event((self.0).0, EpollEventType::In, cx.waker().clone())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for WriteFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = syscall!(write(
            (self.0).0,
            self.1.as_ptr() as *mut libc::c_void,
            self.1.len()
        ));
        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                REACTOR.add_event((self.0).0, EpollEventType::Out, cx.waker().clone())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        info!("(TcpListner) close : {}", self.0);
        syscall!(close(self.0)).ok();
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        info!("(TcpStream)  close : {}", self.0);
        syscall!(close(self.0)).ok();
    }
}