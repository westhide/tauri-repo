mod internal {
    tonic::include_proto!("internal");
}

use crate::{
    grpc_web_client::Client,
    log::{info, instrument},
};
use http::Uri;
use internal::{internal_rpc_client::InternalRpcClient, Username};
use nill::{nil, Nil};
use std::str::FromStr;
use tonic_web::GrpcWebClientLayer;
use tower::ServiceBuilder;

#[instrument(skip_all, err)]
pub async fn call_grpc() -> Result<Nil, Box<dyn std::error::Error>> {
    info!("browser rpc call");

    let service = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(Client::new());

    let uri = Uri::from_str("http://127.0.0.1:3000")?;
    let mut client = InternalRpcClient::with_origin(service, uri);

    let request = tonic::Request::new(Username {
        username: "Tonic".into(),
    });

    let response = client.get_username(request).await?;

    info!(?response);

    Ok(nil)
}
