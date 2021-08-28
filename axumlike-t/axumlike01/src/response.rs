use crate::{
    body::{box_body, BoxBody},
    BoxError, error::Error,
};

use bytes::Bytes;
use http::{header, HeaderMap, HeaderValue, Response, StatusCode};
use http_body::{
    combinators::{MapData, MapErr},
    Empty, Full,
};
use std::{borrow::Cow, convert::Infallible};


pub trait IntoResponse {
    /// The body type of the response.
    ///
    /// Unless you're implementing this trait for a custom body type, these are
    /// some common types you can use:
    ///
    /// - [`axum::body::Body`]: A good default that supports most use cases.
    /// - [`axum::body::Empty<Bytes>`]: When you know your response is always
    /// empty.
    /// - [`axum::body::Full<Bytes>`]: When you know your response always
    /// contains exactly one chunk.
    /// - [`axum::body::BoxBody`]: If you need to unify multiple body types into
    /// one, or return a body type that cannot be named. Can be created with
    /// [`box_body`].
    ///
    /// [`axum::body::Body`]: crate::body::Body
    /// [`axum::body::Empty<Bytes>`]: crate::body::Empty
    /// [`axum::body::Full<Bytes>`]: crate::body::Full
    /// [`axum::body::BoxBody`]: crate::body::BoxBody
    type Body: http_body::Body<Data = Bytes, Error = Self::BodyError> + Send + Sync + 'static;

    /// The error type `Self::Body` might generate.
    ///
    /// Generally it should be possible to set this to:
    ///
    /// ```rust,ignore
    /// type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    /// ```
    ///
    /// This associated type exists mainly to make returning `impl IntoResponse`
    /// possible and to simplify trait bounds internally in axum.
    type BodyError: Into<BoxError>;

    /// Create a response.
    fn into_response(self) -> Response<Self::Body>;
}

impl IntoResponse for () {
    type Body = Empty<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        Response::new(Empty::new())
    }
}

impl IntoResponse for Infallible {
    type Body = Empty<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self {}
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    type Body = BoxBody;
    type BodyError = Error;

    fn into_response(self) -> Response<Self::Body> {
        match self {
            Ok(value) => value.into_response().map(box_body),
            Err(err) => err.into_response().map(box_body),
        }
    }
}

impl<B> IntoResponse for Response<B>
where
    B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
    B::Error: Into<BoxError>,
{
    type Body = B;
    type BodyError = <B as http_body::Body>::Error;

    fn into_response(self) -> Self {
        self
    }
}

impl IntoResponse for &'static str {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    #[inline]
    fn into_response(self) -> Response<Self::Body> {
        Cow::Borrowed(self).into_response()
    }
}

impl IntoResponse for String {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    #[inline]
    fn into_response(self) -> Response<Self::Body> {
        Cow::<'static, str>::Owned(self).into_response()
    }
}

impl IntoResponse for std::borrow::Cow<'static, str> {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Full::from(self));
        res.headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        res
    }
}

impl IntoResponse for Bytes {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Full::from(self));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        res
    }
}

impl IntoResponse for &'static [u8] {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Full::from(self));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        res
    }
}

impl IntoResponse for Vec<u8> {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Full::from(self));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        res
    }
}

impl IntoResponse for std::borrow::Cow<'static, [u8]> {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Full::from(self));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        res
    }
}

impl IntoResponse for StatusCode {
    type Body = Empty<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        Response::builder().status(self).body(Empty::new()).unwrap()
    }
}

impl<T> IntoResponse for (StatusCode, T)
where
    T: IntoResponse,
{
    type Body = T::Body;
    type BodyError = T::BodyError;

    fn into_response(self) -> Response<T::Body> {
        let mut res = self.1.into_response();
        *res.status_mut() = self.0;
        res
    }
}

impl<T> IntoResponse for (HeaderMap, T)
where
    T: IntoResponse,
{
    type Body = T::Body;
    type BodyError = T::BodyError;

    fn into_response(self) -> Response<T::Body> {
        let mut res = self.1.into_response();
        res.headers_mut().extend(self.0);
        res
    }
}

impl<T> IntoResponse for (StatusCode, HeaderMap, T)
where
    T: IntoResponse,
{
    type Body = T::Body;
    type BodyError = T::BodyError;

    fn into_response(self) -> Response<T::Body> {
        let mut res = self.2.into_response();
        *res.status_mut() = self.0;
        res.headers_mut().extend(self.1);
        res
    }
}

impl IntoResponse for HeaderMap {
    type Body = Empty<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(Empty::new());
        *res.headers_mut() = self;
        res
    }
}