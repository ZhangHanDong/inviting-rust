use super::{rejection::*, take_body, FromRequest, RequestParts};
use crate::BoxError;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::Stream;
use http::{Extensions, HeaderMap, Method, Request, Uri, Version};
use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};

#[async_trait]
impl<B> FromRequest<B> for Request<B>
where
    B: Send,
{
    type Rejection = RequestAlreadyExtracted;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let req = std::mem::replace(
            req,
            RequestParts {
                method: req.method.clone(),
                version: req.version,
                uri: req.uri.clone(),
                headers: None,
                extensions: None,
                body: None,
            },
        );

        let err = match req.try_into_request() {
            Ok(req) => return Ok(req),
            Err(err) => err,
        };

        match err.downcast::<RequestAlreadyExtracted>() {
            Ok(err) => return Err(err),
            Err(err) => unreachable!(
                "Unexpected error type from `try_into_request`: `{:?}`. This is a bug in axum, please file an issue",
                err,
            ),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Body<B = crate::body::Body>(pub B);

#[async_trait]
impl<B> FromRequest<B> for Body<B>
where
    B: Send,
{
    type Rejection = BodyAlreadyExtracted;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let body = take_body(req)?;
        Ok(Self(body))
    }
}

#[async_trait]
impl<B> FromRequest<B> for Method
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(req.method().clone())
    }
}

#[async_trait]
impl<B> FromRequest<B> for Uri
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(req.uri().clone())
    }
}


#[derive(Debug)]
pub struct BodyStream<B = crate::body::Body>(B);

impl<B> Stream for BodyStream<B>
where
    B: http_body::Body + Unpin,
{
    type Item = Result<B::Data, B::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_data(cx)
    }
}

#[async_trait]
impl<B> FromRequest<B> for BodyStream<B>
where
    B: http_body::Body + Unpin + Send,
{
    type Rejection = BodyAlreadyExtracted;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let body = take_body(req)?;
        let stream = BodyStream(body);
        Ok(stream)
    }
}

#[async_trait]
impl<B> FromRequest<B> for String
where
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = StringRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let body = take_body(req)?;

        let bytes = hyper::body::to_bytes(body)
            .await
            .map_err(FailedToBufferBody::from_err)?
            .to_vec();

        let string = String::from_utf8(bytes).map_err(InvalidUtf8::from_err)?;

        Ok(string)
    }
}