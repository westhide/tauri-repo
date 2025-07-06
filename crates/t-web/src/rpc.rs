use crate::{
    grpc_web_client::Client,
    log::{info, instrument},
};
use http::Uri;
use std::str::FromStr;
use t_rpc::{Username, internal_rpc_client::InternalRpcClient};
use tonic::Request;
use tonic_web::GrpcWebClientLayer;
use tower::ServiceBuilder;

#[instrument(skip_all, err)]
pub async fn get_username(username: String) -> Result<String, Box<dyn std::error::Error>> {
    info!("browser rpc call");

    let service = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(Client::new());

    let uri = Uri::from_str("http://127.0.0.1:3000")?;
    let mut client = InternalRpcClient::with_origin(service, uri);

    let request = Request::new(Username { username });

    let response = client.get_username(request).await?;

    info!(?response);

    Ok(response.into_inner().username)
}
