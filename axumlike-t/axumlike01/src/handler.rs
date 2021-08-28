use std::{
    convert::Infallible,
    fmt,
    future::Future,
    marker::PhantomData,
    task::{Context, Poll},
};

use crate::{
    body::{box_body, BoxBody}, 
    router::empty_router::EmptyRouter,
    util::Either,
};

use crate::response::IntoResponse;
use tower_service::Service;

use http::{Request, Response, StatusCode, Uri};

use async_trait::async_trait;

use crate::router::MethodFilter;

use self::into_service::IntoService;
use tower::ServiceExt;
mod into_service;
mod future;

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
    // This seals the trait. We cannot use the regular "sealed super trait"
    // approach due to coherence.
    #[doc(hidden)]
    type Sealed: sealed::HiddentTrait;

    /// Call the handler with the given request.
    async fn call(self, req: Request<B>) -> Response<BoxBody>;

    

    /// Convert the handler into a [`Service`].
    ///
    /// This allows you to serve a single handler if you don't need any routing:
    ///
    /// ```rust
    /// use axum::{
    ///     Server, handler::Handler, http::{Uri, Method}, response::IntoResponse,
    /// };
    /// use tower::make::Shared;
    /// use std::net::SocketAddr;
    ///
    /// async fn handler(method: Method, uri: Uri, body: String) -> impl IntoResponse {
    ///     format!("received `{} {}` with body `{:?}`", method, uri, body)
    /// }
    ///
    /// let service = handler.into_service();
    ///
    /// # async {
    /// Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
    ///     .serve(Shared::new(service))
    ///     .await?;
    /// # Ok::<_, hyper::Error>(())
    /// # };
    /// ```
    fn into_service(self) -> IntoService<Self, B, T> {
        IntoService::new(self)
    }
}

#[async_trait]
impl<F, Fut, Res, B> Handler<B, ()> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    B: Send + 'static,
{
    type Sealed = sealed::Hidden;

    async fn call(self, _req: Request<B>) -> Response<BoxBody> {
        self().await.into_response().map(box_body)
    }
}


impl<H, B, T, F> Clone for OnMethod<H, B, T, F>
where
    H: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            method: self.method,
            handler: self.handler.clone(),
            fallback: self.fallback.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, B, T, F> Copy for OnMethod<H, B, T, F>
where
    H: Copy,
    F: Copy,
{
}

impl<H, B, T, F> OnMethod<H, B, T, F> {
    /// Chain an additional handler that will accept all requests regardless of
    /// its HTTP method.
    ///
    /// See [`OnMethod::get`] for an example.
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

    /// Chain an additional handler that will only accept `DELETE` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn delete<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::DELETE, handler)
    }

    /// Chain an additional handler that will only accept `GET` requests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::{handler::post, Router};
    ///
    /// async fn handler() {}
    ///
    /// async fn other_handler() {}
    ///
    /// // Requests to `GET /` will go to `handler` and `POST /` will go to
    /// // `other_handler`.
    /// let app = Router::new().route("/", post(handler).get(other_handler));
    /// # async {
    /// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    /// # };
    /// ```
    ///
    /// Note that `get` routes will also be called for `HEAD` requests but will have
    /// the response body removed. Make sure to add explicit `HEAD` routes
    /// afterwards.
    pub fn get<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::GET | MethodFilter::HEAD, handler)
    }

    /// Chain an additional handler that will only accept `HEAD` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn head<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::HEAD, handler)
    }

    /// Chain an additional handler that will only accept `OPTIONS` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn options<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::OPTIONS, handler)
    }

    /// Chain an additional handler that will only accept `PATCH` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn patch<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::PATCH, handler)
    }

    /// Chain an additional handler that will only accept `POST` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn post<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::POST, handler)
    }

    /// Chain an additional handler that will only accept `PUT` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn put<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::PUT, handler)
    }

    /// Chain an additional handler that will only accept `TRACE` requests.
    ///
    /// See [`OnMethod::get`] for an example.
    pub fn trace<H2, T2>(self, handler: H2) -> OnMethod<H2, B, T2, Self>
    where
        H2: Handler<B, T2>,
    {
        self.on(MethodFilter::TRACE, handler)
    }

    /// Chain an additional handler that will accept requests matching the given
    /// `MethodFilter`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::{
    ///     handler::get,
    ///     Router,
    ///     routing::MethodFilter
    /// };
    ///
    /// async fn handler() {}
    ///
    /// async fn other_handler() {}
    ///
    /// // Requests to `GET /` will go to `handler` and `DELETE /` will go to
    /// // `other_handler`
    /// let app = Router::new().route("/", get(handler).on(MethodFilter::DELETE, other_handler));
    /// # async {
    /// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    /// # };
    /// ```
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


impl<H, B, T, F> fmt::Debug for OnMethod<H, B, T, F>
where
    T: fmt::Debug,
    F: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnMethod")
            .field("method", &self.method)
            .field("handler", &format_args!("{}", std::any::type_name::<H>()))
            .field("fallback", &self.fallback)
            .finish()
    }
}
