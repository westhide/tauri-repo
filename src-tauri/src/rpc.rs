mod internal {
    tonic::include_proto!("internal");
}

use crate::log::{info, instrument};
use internal::{
    internal_rpc_server::{InternalRpc, InternalRpcServer},
    Username,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct InternalRpcImpl {}

#[tonic::async_trait]
impl InternalRpc for InternalRpcImpl {
    #[instrument(ret, err)]
    async fn get_username(&self, req: Request<Username>) -> Result<Response<Username>, Status> {
        info!("Got a request: {:?}", req);

        let rep = Username {
            username: format!("Username: {}", req.into_inner().username),
        };

        Ok(Response::new(rep))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("rpc run");

    let addr = "127.0.0.1:3000".parse()?;
    let service = InternalRpcServer::new(InternalRpcImpl::default());

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::new())
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
