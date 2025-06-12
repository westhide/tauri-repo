mod internal {
    tonic::include_proto!("internal");
}

// use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use crate::{
    http_client::HttpClient,
    log::{info, instrument},
};
use http::Uri;
use internal::{internal_rpc_client::InternalRpcClient, Username};
use nill::{nil, Nil};
use tonic_web::GrpcWebClientLayer;
use tower::ServiceBuilder;

#[instrument]
pub async fn call_grpc() -> Result<Nil, Box<dyn std::error::Error>> {
    info!("browser rpc call");

    let uri = Uri::try_from("http://127.0.0.1:3000")?;
    let service = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(HttpClient::new());

    let mut client = InternalRpcClient::with_origin(service, uri);

    let request = tonic::Request::new(Username {
        username: "Tonic".into(),
    });

    let response = client.get_username(request).await?;

    Ok(nil)
}
