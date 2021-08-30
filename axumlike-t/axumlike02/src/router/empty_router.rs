use super::*;


// 创建空路由
// Infallible
// 代表不可能发生错误的错误类型，因为这是个空枚举，
// 这对使用Result并对错误类型进行参数化的通用API很有用，以表明结果总是OK。
pub struct EmptyRouter<E = Infallible> {
    // http 中定义的状态码
    status: StatusCode,
    // 因为定义用到泛型 E，而字段里则没有，所以这里使用 PhatomData来标记
    // 这里 E 并不被 EmptyRouter 所有，使用 `PhantomData<fn() -> E>`来跳过所有权
    _marker: PhantomData<fn() -> E>,
}

impl<E> EmptyRouter<E> {
    pub(crate) fn not_found() -> Self {
        Self {
            // 404
            status: StatusCode::NOT_FOUND,
            _marker: PhantomData,
        }
    }

    pub(crate) fn method_not_allowed() -> Self {
        Self {
            // 405
            status: StatusCode::METHOD_NOT_ALLOWED,
            _marker: PhantomData,
        }
    }
}

impl<E> Clone for EmptyRouter<E> {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            _marker: PhantomData,
        }
    }
}

impl<E> fmt::Debug for EmptyRouter<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EmptyRouter").finish()
    }
}

// 为 EmptyRouter 实现 Service

impl<B, E> Service<Request<B>> for EmptyRouter<E>
where
    B: Send + Sync + 'static,
{
    type Response = Response<BoxBody>;
    type Error = E;
    type Future = EmptyRouterFuture<E>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        if self.status == StatusCode::METHOD_NOT_ALLOWED {
            // 没有方法匹配，记录到 extension 中
            request.extensions_mut().insert(NoMethodMatch);
        }

        if self.status == StatusCode::NOT_FOUND
            && request.extensions().get::<NoMethodMatch>().is_some()
        {
            self.status = StatusCode::METHOD_NOT_ALLOWED;
        }

        let mut res = Response::new(crate::body::empty());

        res.extensions_mut().insert(FromEmptyRouter { request });

        *res.status_mut() = self.status;
        EmptyRouterFuture {
            future: ready(Ok(res)),
        }
    }
}

#[derive(Clone, Copy)]
struct NoMethodMatch;

pub struct FromEmptyRouter<B> {
    pub request: Request<B>,
}
