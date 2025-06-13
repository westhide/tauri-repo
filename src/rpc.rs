mod internal {
    tonic::include_proto!("internal");
}

mod helloworld {
    tonic::include_proto!("helloworld");
}

use std::str::FromStr;

use crate::{
    grpc_web_client::Client,
    log::{info, instrument},
};
use http::Uri;
// use internal::{internal_rpc_client::InternalRpcClient, Username};
use nill::{nil, Nil};
use tonic_web::GrpcWebClientLayer;
use tower::ServiceBuilder;

use helloworld::{greeter_client::GreeterClient, HelloRequest};

// #[instrument]
// pub async fn call_grpc() -> Result<Nil, Box<dyn std::error::Error>> {
//     info!("browser rpc call");

//     let uri = Uri::try_from("http://127.0.0.1:3000")?;
//     let service = ServiceBuilder::new()
//         .layer(GrpcWebClientLayer::new())
//         .service(Client::new());

//     let mut client = InternalRpcClient::with_origin(service, uri);

//     let request = tonic::Request::new(Username {
//         username: "Tonic".into(),
//     });

//     let response = client.get_username(request).await?;

//     Ok(nil)
// }

#[instrument]
pub async fn call_grpc() -> Result<Nil, Box<dyn std::error::Error>> {
    info!("browser rpc call");

    let base_url = "http://127.0.0.1:3000";
    let service = ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(Client::new(base_url.into()));

    let mut client = GreeterClient::with_origin(service, Uri::from_str(base_url)?);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    Ok(nil)
}
