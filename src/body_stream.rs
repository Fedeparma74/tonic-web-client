use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{stream::empty, Stream, TryStreamExt};
use http_body::Body;

use crate::Error;

pub struct BodyStream {
    body_stream: Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>>,
}

impl BodyStream {
    pub fn new(body_stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + 'static) -> Self {
        let body_stream = body_stream.map_err(Error::from);

        Self {
            body_stream: Box::pin(body_stream),
        }
    }

    pub fn empty() -> Self {
        let body_stream = empty();

        Self {
            body_stream: Box::pin(body_stream),
        }
    }
}

impl Body for BodyStream {
    type Data = Bytes;

    type Error = Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        self.body_stream.as_mut().poll_next(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

unsafe impl Send for BodyStream {}
unsafe impl Sync for BodyStream {}
