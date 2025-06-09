use crate::log::info;
use internal::{
    internal_rpc_server::{InternalRpc, InternalRpcServer},
    Username,
};
use tonic::{transport::Server, Request, Response, Status};

mod internal {
    tonic::include_proto!("internal");
}

#[derive(Debug, Default)]
pub struct InternalRpcImpl {}

#[tonic::async_trait]
impl InternalRpc for InternalRpcImpl {
    async fn get_username(&self, req: Request<Username>) -> Result<Response<Username>, Status> {
        println!("Got a request: {:?}", req);

        let rep = Username {
            username: format!("Username: {}", req.into_inner().username),
        };

        Ok(Response::new(rep))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let internal_service = InternalRpcServer::new(InternalRpcImpl::default());

    info!("rpc run");

    Server::builder()
        .accept_http1(true)
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(internal_service)
        .serve("127.0.0.1:3000".parse()?)
        .await?;

    Ok(())
}
