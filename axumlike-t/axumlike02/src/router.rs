pub mod empty_router;
pub mod route;
pub mod future;
pub mod method_filter;

use crate::buffer::MpscBuffer;

use std::{
    borrow::Cow,
    convert::Infallible,
    fmt,
    future::ready,
    marker::PhantomData,
    sync::Arc,
    task::{Context, Poll},
};

use crate::body::{box_body, BoxBody,};

use crate::service::HandleError;

use http::{Request, Response, StatusCode, Uri};
use tower::{
    util::{BoxService, ServiceExt},
    ServiceBuilder,
};
use tower_http::map_response_body::MapResponseBodyLayer;
use tower_layer::Layer;
use tower_service::Service;

use crate::BoxError;

use self::{
    empty_router::{EmptyRouter, FromEmptyRouter},
    route::{PathPattern, Route},
    future::{EmptyRouterFuture, RouteFuture},
};

pub use self::method_filter::MethodFilter;

#[derive(Debug, Clone)]
pub struct Router<S> {
    // 代表 Service
    svc: S,
}

impl<E> Router<EmptyRouter<E>> {
    // 创建一个新的路由，默认是 Not Found
    pub fn new() -> Self {
        Self {
            svc: EmptyRouter::not_found(),
        }
    }
}

impl<E> Default for Router<EmptyRouter<E>> {
    fn default() -> Self {
        Self::new()
    }
}

// 为 Router 实现 Service
impl<S, R> Service<R> for Router<S>
where
    S: Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.svc.call(req)
    }
}

impl<S> Router<S> {
    pub fn route<T>(self, description: &str, svc: T) -> Router<Route<T, S>> {
        self.map(|fallback| Route {
            pattern: PathPattern::new(description),
            svc,
            fallback,
        })
    }

    fn map<F, S2>(self, f: F) -> Router<S2>
    where
        F: FnOnce(S) -> S2,
    {
        Router { svc: f(self.svc) }
    }

    pub fn into_make_service(self) -> IntoMakeService<S>
    where
        S: Clone,
    {
        IntoMakeService::new(self.svc)
    }

    // 使用 tower_service::Layer 为 route 增加中间件支持
    pub fn layer<L>(self, layer: L) -> Router<Layered<L::Service>>
    where
        L: Layer<S>,
    {
        self.map(|svc| Layered::new(layer.layer(svc)))
    }

    pub fn handle_error<ReqBody, F>(self, f: F) -> Router<HandleError<S, F, ReqBody>> {
        self.map(|svc| HandleError::new(svc, f))
    }

    /// Check that your service cannot fail.
    ///
    /// That is, its error type is [`Infallible`].
    pub fn check_infallible(self) -> Router<CheckInfallible<S>> {
        self.map(CheckInfallible)
    }
}

/// A [`MakeService`] that produces axum router services.
///
/// [`MakeService`]: tower::make::MakeService
#[derive(Debug, Clone)]
pub struct IntoMakeService<S> {
    service: S,
}

impl<S> IntoMakeService<S> {
    fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S, T> Service<T> for IntoMakeService<S>
where
    S: Clone,
{
    type Response = S;
    type Error = Infallible;
    type Future = future::MakeRouteServiceFuture<S>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _target: T) -> Self::Future {
        future::MakeRouteServiceFuture {
            future: ready(Ok(self.service.clone())),
        }
    }
}


/// A [`Service`] created from a router by applying a Tower middleware.
///
/// Created with [`Router::layer`]. See that method for more details.
pub struct Layered<S> {
    inner: S,
}

impl<S> Layered<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Clone for Layered<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl<S> fmt::Debug for Layered<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layered")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<S, R> Service<R> for Layered<S>
where
    S: Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.inner.call(req)
    }
}


/// Middleware that statically verifies that a service cannot fail.
///
/// Created with [`check_infallible`](Router::check_infallible).
#[derive(Debug, Clone, Copy)]
pub struct CheckInfallible<S>(S);

impl<R, S> Service<R> for CheckInfallible<S>
where
    S: Service<R, Error = Infallible>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.0.call(req)
    }
}