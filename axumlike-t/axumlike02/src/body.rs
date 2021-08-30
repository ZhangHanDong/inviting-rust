// 引入 http_body 中的 Body trait 及其 trait对象
// 用于一致性地处理请求或响应中的 Body 

pub use http_body::{Body as HttpBody, Empty, Full};
// hyper 中定义的结构体，用于接收字节流
pub use hyper::body::Body;
pub use bytes::Bytes;

use crate::error::Error;
use crate::BoxError;

pub type BoxBody = http_body::combinators::BoxBody<Bytes, Error>;

/// 把 `http_body::Body` 转为 `BoxBody`
pub fn box_body<B>(body: B) -> BoxBody
where
    B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    B::Error: Into<BoxError>,
{
    body.map_err(Error::new).boxed()
}

// 空body
pub(crate) fn empty() -> BoxBody {
    box_body(http_body::Empty::new())
}

