use std::io;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use async_trait::async_trait;
use futures_util::io::{AsyncRead, AsyncWrite};
use http::{Request, Response};

use crate::body::{DecoderBody, EncoderBody};
use crate::decoder::{Http1RequestDecoder, Http1ResponseDecoder};
use crate::encoder::{Http1RequestEncoder, Http1ResponseEncoder};
use crate::message::{body_framing::BodyFraming, head_renderer::Head, ReasonPhrase};

//
//
//
#[async_trait]
pub trait Http1StreamDecoder<S, H>
where
    S: AsyncRead + Unpin,
    H: Head,
{
    async fn read_head(&mut self, stream: &mut S) -> io::Result<H>;
    async fn read_body(&mut self, stream: &mut S) -> io::Result<DecoderBody>;

    fn set_read_timeout(&mut self, dur: Duration);
    fn has_unparsed_bytes(&self) -> bool;
}

#[async_trait]
pub trait Http1StreamEncoder<S, H>
where
    S: AsyncWrite + Unpin,
    H: Head,
{
    async fn write_head(
        &mut self,
        stream: &mut S,
        head: H,
        body_framing: BodyFraming,
    ) -> io::Result<()>;
    async fn write_body(&mut self, stream: &mut S, body: EncoderBody) -> io::Result<()>;

    fn set_write_timeout(&mut self, dur: Duration);
}

//
//
//
pub struct Http1Stream<S, D, DH, E, EH>
where
    S: AsyncRead + AsyncWrite + Unpin,
    D: Http1StreamDecoder<S, DH>,
    DH: Head,
    E: Http1StreamEncoder<S, EH>,
    EH: Head,
{
    stream: S,
    decoder: D,
    encoder: E,
    phantom: PhantomData<(DH, EH)>,
}
impl<S, D, DH, E, EH> Http1Stream<S, D, DH, E, EH>
where
    S: AsyncRead + AsyncWrite + Unpin,
    D: Http1StreamDecoder<S, DH>,
    DH: Head,
    E: Http1StreamEncoder<S, EH>,
    EH: Head,
{
    pub(crate) fn new(stream: S, decoder: D, encoder: E) -> Self {
        Self {
            stream,
            decoder,
            encoder,
            phantom: PhantomData,
        }
    }

    //
    pub fn set_write_timeout(&mut self, dur: Duration) {
        self.encoder.set_write_timeout(dur)
    }

    pub fn set_read_timeout(&mut self, dur: Duration) {
        self.decoder.set_read_timeout(dur)
    }

    //
    pub fn get_ref(&self) -> &S {
        &self.stream
    }
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }
    pub fn into_inner(self) -> io::Result<S> {
        if self.decoder.has_unparsed_bytes() {
            return Err(io::Error::new(io::ErrorKind::Other, "has unparsed bytes"));
        }
        Ok(self.stream)
    }

    //
    pub async fn write_head(&mut self, head: EH, body_framing: BodyFraming) -> io::Result<()> {
        self.encoder
            .write_head(&mut self.stream, head, body_framing)
            .await
    }

    pub async fn write_body(&mut self, body: EncoderBody) -> io::Result<()> {
        self.encoder.write_body(&mut self.stream, body).await
    }

    //
    pub async fn read_head(&mut self) -> io::Result<DH> {
        self.decoder.read_head(&mut self.stream).await
    }
    pub async fn read_body(&mut self) -> io::Result<DecoderBody> {
        self.decoder.read_body(&mut self.stream).await
    }
}

//
//
//
pub type Http1ClientStreamInner<S> = Http1Stream<
    S,
    Http1ResponseDecoder,
    (Response<()>, ReasonPhrase),
    Http1RequestEncoder,
    Request<()>,
>;
pub struct Http1ClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    inner: Http1ClientStreamInner<S>,
}
impl<S> Deref for Http1ClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    type Target = Http1ClientStreamInner<S>;

    fn deref(&self) -> &Http1ClientStreamInner<S> {
        &self.inner
    }
}
impl<S> DerefMut for Http1ClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn deref_mut(&mut self) -> &mut Http1ClientStreamInner<S> {
        &mut self.inner
    }
}
impl<S> Http1ClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self::with(
            stream,
            Http1ResponseDecoder::new(8 * 1024, None),
            Http1RequestEncoder::new(8 * 1024),
        )
    }
    pub fn with(stream: S, decoder: Http1ResponseDecoder, encoder: Http1RequestEncoder) -> Self {
        Self {
            inner: Http1ClientStreamInner::new(stream, decoder, encoder),
        }
    }

    pub async fn write_request(&mut self, request: Request<Vec<u8>>) -> io::Result<()> {
        let (parts, body) = request.into_parts();
        let head = Request::from_parts(parts, ());

        self.write_head(head, BodyFraming::ContentLength(body.len()))
            .await?;
        self.write_body(EncoderBody::Completed(body)).await?;

        Ok(())
    }

    pub async fn read_response(&mut self) -> io::Result<(Response<Vec<u8>>, ReasonPhrase)> {
        let (response, reason_phrase) = self.read_head().await?;

        let mut body = Vec::new();
        loop {
            match self.read_body().await? {
                DecoderBody::Completed(data) => {
                    body.extend_from_slice(&data);
                    break;
                }
                DecoderBody::Partial(data) => {
                    body.extend_from_slice(&data);
                }
            }
        }

        let (parts, _) = response.into_parts();
        let response = Response::from_parts(parts, body);

        Ok((response, reason_phrase))
    }
}

//
//
//
pub type Http1ServerStreamInner<S> = Http1Stream<
    S,
    Http1RequestDecoder,
    Request<()>,
    Http1ResponseEncoder,
    (Response<()>, ReasonPhrase),
>;
pub struct Http1ServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    inner: Http1ServerStreamInner<S>,
}
impl<S> Deref for Http1ServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    type Target = Http1ServerStreamInner<S>;

    fn deref(&self) -> &Http1ServerStreamInner<S> {
        &self.inner
    }
}
impl<S> DerefMut for Http1ServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    fn deref_mut(&mut self) -> &mut Http1ServerStreamInner<S> {
        &mut self.inner
    }
}
impl<S> Http1ServerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self::with(
            stream,
            Http1RequestDecoder::new(8 * 1024, None),
            Http1ResponseEncoder::new(8 * 1024),
        )
    }
    pub fn with(stream: S, decoder: Http1RequestDecoder, encoder: Http1ResponseEncoder) -> Self {
        Self {
            inner: Http1ServerStreamInner::new(stream, decoder, encoder),
        }
    }

    pub async fn write_response(
        &mut self,
        response: Response<Vec<u8>>,
        reason_phrase: ReasonPhrase,
    ) -> io::Result<()> {
        let (parts, body) = response.into_parts();
        let head = Response::from_parts(parts, ());

        self.write_head(
            (head, reason_phrase),
            BodyFraming::ContentLength(body.len()),
        )
        .await?;
        self.write_body(EncoderBody::Completed(body)).await?;

        Ok(())
    }

    pub async fn read_request(&mut self) -> io::Result<Request<Vec<u8>>> {
        let request = self.read_head().await?;

        let mut body = Vec::new();
        loop {
            match self.read_body().await? {
                DecoderBody::Completed(data) => {
                    body.extend_from_slice(&data);
                    break;
                }
                DecoderBody::Partial(data) => {
                    body.extend_from_slice(&data);
                }
            }
        }

        let (parts, _) = request.into_parts();
        let request = Request::from_parts(parts, body);

        Ok(request)
    }
}
