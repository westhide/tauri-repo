use crate::log::{debug, info, instrument};
use gloo::net::http::{
    Headers as GlooHttpHeaders, Method, Request as GlooHttpRequest,
    RequestBuilder as GlooHttpRequestBuilder, Response as GlooHttpResponse,
};
use http::{
    header::{
        InvalidHeaderName as HttpInvalidHeaderName, InvalidHeaderValue as HttpInvalidHeaderValue,
        ToStrError as HttpHeaderToStrError,
    },
    Error as HttpError, HeaderName, Request as HttpRequest, Response as HttpResponse,
};
use http_body_util::BodyExt;
use js_sys::Uint8Array;
use nill::{nil, Nil};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tonic::{body::Body as TonicBody, Status};
use tonic_web::GrpcWebCall;
use tower::Service;
use web_sys::RequestMode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    HttpError(#[from] HttpError),

    #[error(transparent)]
    HttpHeaderToStr(#[from] HttpHeaderToStrError),

    #[error(transparent)]
    HttpInvalidHeaderName(#[from] HttpInvalidHeaderName),

    #[error(transparent)]
    HttpInvalidHeaderValue(#[from] HttpInvalidHeaderValue),

    #[error(transparent)]
    TonicStatus(#[from] Status),

    #[error(transparent)]
    GlooNet(#[from] gloo::net::Error),
}

trait HttpRequestExt {
    async fn try_into_fetch(self) -> Result<GlooHttpRequest, Error>;
}

impl HttpRequestExt for HttpRequest<GrpcWebCall<TonicBody>> {
    async fn try_into_fetch(self) -> Result<GlooHttpRequest, Error> {
        let uri = self.uri().to_string();
        let headers = GlooHttpHeaders::new();
        info!("=================== {:?}", self.headers());
        for (key, val) in self.headers() {
            headers.set(key.as_str(), val.to_str()?);
        }
        let bytes = self.into_body().collect().await?.to_bytes();
        let fetch = GlooHttpRequestBuilder::new(&uri)
            .mode(RequestMode::Cors)
            .headers(headers)
            .method(Method::POST)
            .body(Uint8Array::from(&*bytes))?;
        Ok(fetch)
    }
}

trait HttpResponseExt {
    async fn try_into_grpc(self) -> Result<HttpResponse<TonicBody>, Error>;
}

impl HttpResponseExt for GlooHttpResponse {
    async fn try_into_grpc(self) -> Result<HttpResponse<TonicBody>, Error> {
        let status = self.status();
        // let body = fetch.body().unwrap();
        let body = TonicBody::empty();
        let mut resp = HttpResponse::builder().status(status).body(body)?;

        let headers = resp.headers_mut();
        for (key, val) in self.headers().entries() {
            headers.insert(HeaderName::try_from(key)?, val.parse()?);
        }
        debug!("headers: {:?}", resp.headers());
        Ok(resp)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Client { base_url }
    }

    #[instrument(skip_all, ret, err, fields(url = ?grpc.uri()))]
    async fn grpc_web_call(
        self,
        grpc: HttpRequest<GrpcWebCall<TonicBody>>,
    ) -> Result<HttpResponse<TonicBody>, Error> {
        let fetch = grpc.try_into_fetch().await?;
        let resp = fetch.send().await?;
        info!("[RESP]: {resp:?}");
        resp.try_into_grpc().await
    }
}

impl Service<HttpRequest<GrpcWebCall<TonicBody>>> for Client {
    type Response = HttpResponse<TonicBody>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<Nil, Self::Error>> {
        Poll::Ready(Ok(nil))
    }

    fn call(&mut self, grpc: HttpRequest<GrpcWebCall<TonicBody>>) -> Self::Future {
        // TODO: void clone
        Box::pin(self.clone().grpc_web_call(grpc))
    }
}
