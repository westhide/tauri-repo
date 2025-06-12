use crate::log::{info, instrument};
use http::{Request, Response};
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tonic::body::Body;
use tonic_web::GrpcWebCall;
use tower::Service;

#[derive(Debug, Clone)]
pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    #[instrument(skip_all, ret, err, fields(url = ?request.uri()))]
    async fn call_impl(
        self,
        request: Request<GrpcWebCall<Body>>,
    ) -> Result<Response<Body>, FetchError> {
        todo!()
    }

    //     async fn request(self, rpc: Request<BoxBody>) -> Result<Response<BoxBody>, ClientError> {
    //         let mut uri = rpc.uri().to_string();
    //         uri.insert_str(0, &self.base_uri);

    //         let headers = Headers::new().unwrap();
    //         for (k, v) in rpc.headers().iter() {
    //             headers.set(k.as_str(), v.to_str().unwrap()).unwrap();
    //         }
    //         headers.set("x-user-agent", "grpc-web-rust/0.1").unwrap();
    //         headers.set("x-grpc-web", "1").unwrap();
    //         headers
    //             .set("content-type", self.encoding.to_content_type())
    //             .unwrap();

    //         let body_bytes = hyper::body::to_bytes(rpc.into_body()).await.unwrap();
    //         let body_array: Uint8Array = body_bytes.as_ref().into();
    //         let body_js: &JsValue = body_array.as_ref();

    //         let mut init = RequestInit::new();
    //         init.method("POST")
    //             .mode(self.mode)
    //             .credentials(self.credentials)
    //             .body(Some(body_js))
    //             .headers(headers.as_ref());

    //         let request = web_sys::Request::new_with_str_and_init(&uri, &init).unwrap();

    //         let window = web_sys::window().unwrap();
    //         let fetch = JsFuture::from(window.fetch_with_request(&request))
    //             .await
    //             .map_err(ClientError::FetchFailed)?;
    //         let fetch_res: web_sys::Response = fetch.dyn_into().unwrap();

    //         let mut res = Response::builder().status(fetch_res.status());
    //         let headers = res.headers_mut().unwrap();

    //         for kv in js_sys::try_iter(fetch_res.headers().as_ref())
    //             .unwrap()
    //             .unwrap()
    //         {
    //             let pair: Array = kv.unwrap().into();
    //             headers.append(
    //                 HeaderName::from_bytes(pair.get(0).as_string().unwrap().as_bytes()).unwrap(),
    //                 HeaderValue::from_str(&pair.get(1).as_string().unwrap()).unwrap(),
    //             );
    //         }

    //         let body_stream = ReadableStream::from_raw(fetch_res.body().unwrap().unchecked_into());
    //         let body = GrpcWebCall::client_response(
    //             ReadableStreamBody::new(body_stream),
    //             Encoding::from_content_type(headers),
    //         );

    //         Ok(res.body(BoxBody::new(body)).unwrap())
    //     }
}

#[derive(Debug)]
pub struct FetchError {}

impl Error for FetchError {}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Service<Request<GrpcWebCall<Body>>> for Client {
    type Response = Response<Body>;

    type Error = FetchError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<GrpcWebCall<Body>>) -> Self::Future {
        Box::pin(self.clone().call_impl(request))
    }
}
