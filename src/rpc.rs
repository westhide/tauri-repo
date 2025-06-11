mod internal {
    tonic::include_proto!("internal");
}

use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use internal::{internal_rpc_client::InternalRpcClient, Username};
use tonic_web::GrpcWebClientLayer;

pub async fn call_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(TokioExecutor::new()).build_http();

    let svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client = InternalRpcClient::with_origin(svc, "http://127.0.0.1:3000".try_into()?);

    let request = tonic::Request::new(Username {
        username: "Tonic".into(),
    });

    let response = client.get_username(request).await?;

    Ok(())
}
