mod internal {
    tonic::include_proto!("internal");
}

// use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use crate::{
    log::{info, instrument},
    rpc_client::Client,
};
use internal::{internal_rpc_client::InternalRpcClient, Username};
use tonic_web::GrpcWebClientLayer;
use tower::ServiceBuilder;

#[instrument]
pub async fn call_grpc() -> Result<(), Box<dyn std::error::Error>> {
    info!("browser rpc call");

    let service = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(Client::new());

    let mut client = InternalRpcClient::with_origin(service, "http://127.0.0.1:3000".try_into()?);

    let request = tonic::Request::new(Username {
        username: "Tonic".into(),
    });

    let response = client.get_username(request).await?;

    Ok(())
}
