mod internal {
    tonic::include_proto!("internal");
}

use crate::log::{info, instrument};
use internal::{
    internal_rpc_server::{InternalRpc, InternalRpcServer},
    Username,
};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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
        // .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
