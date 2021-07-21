
// 也许是 Rust 网络生态中被复制最多的一段代码！
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

#[link(name = "c")]
extern "C" {
    /// htons: H(host byte order) TO N(network byte order) L(Long)
    /// 为了考虑不同操作系统和处理器的大端序小端序可能不同，所以都转成统一的默认的 network byte order
    pub fn htonl(hostlong: u32) -> u32;
}
