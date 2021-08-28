use async_trait::async_trait;

pub(crate) mod sealed {
    #![allow(unreachable_pub, missing_docs, missing_debug_implementations)]

    pub trait HiddentTrait {}
    pub struct Hidden;
    impl HiddentTrait for Hidden {}
}

// Handler 是一个异步 trait
// 框架用户不应该依赖这个 trait，所以用 Sealed 封装起来
// 会为 正确类型的闭包 自动实现
#[async_trait]
pub trait Handler<B, T>: Clone + Send + Sized + 'static {
    #[doc(hidden)]
    type Sealed: sealed::HiddentTrait;

    async fn call(self, req: Request<B>) -> Response<BoxBody>;

    fn into_service(self) -> IntoService<Self, B, T> {
        IntoService::new(self)
    }
}

pub struct OnMethod<H, B, T, F> {
    pub(crate) method: MethodFilter,
    pub(crate) handler: H,
    pub(crate) fallback: F,
    pub(crate) _marker: PhantomData<fn() -> (B, T)>,
}

pub fn on<H, B, T>(method: MethodFilter, handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    OnMethod {
        method,
        handler,
        fallback: EmptyRouter::method_not_allowed(),
        _marker: PhantomData,
    }
}

pub fn get<H, B, T>(handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    on(MethodFilter::GET | MethodFilter::HEAD, handler)
}

pub fn post<H, B, T>(handler: H) -> OnMethod<H, B, T, EmptyRouter>
where
    H: Handler<B, T>,
{
    on(MethodFilter::POST, handler)
}


impl<H, B, T, F> OnMethod<H, B, T, F> {

    pub fn any<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::all(), handler)
    }

    /// Chain an additional handler that will only accept `CONNECT` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn connect<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::CONNECT, handler)
    }

    pub fn delete<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::DELETE, handler)
    }

    pub fn get<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::GET | MethodFilter::HEAD, handler)
    }

    pub fn on<H2, T2>(self, method: MethodFilter, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        OnMethod {
            method,
            handler,
            fallback: self,
            _marker: PhantomData,
        }
    }
}

impl<H, B, T, F> Service<Request<B>> for OnMethod<H, B, T, F>
where
    H: Handler<B, T>,
    F: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible> + Clone,
    B: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = future::OnMethodFuture<F, B>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let req_method = req.method().clone();

        let fut = if self.method.matches(req.method()) {
            let fut = Handler::call(self.handler.clone(), req);
            Either::A { inner: fut }
        } else {
            let fut = self.fallback.clone().oneshot(req);
            Either::B { inner: fut }
        };

        future::OnMethodFuture {
            inner: fut,
            req_method,
        }
    }
}
