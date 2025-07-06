use nill::{Nil, nil};
use t_lib::{
    error::Result,
    log::{info, instrument},
};
use t_rpc::{
    Request, Response, Status,
    proto::{
        Username,
        internal_rpc_server::{InternalRpc, InternalRpcServer},
    },
    transport::Server,
    web::GrpcWebLayer,
};
use tower_http::cors::CorsLayer;

#[derive(Debug, Default)]
pub struct InternalRpcImpl {}

#[t_rpc::async_trait]
impl InternalRpc for InternalRpcImpl {
    #[instrument(skip_all, err)]
    async fn get_username(&self, req: Request<Username>) -> Result<Response<Username>, Status> {
        Ok(Response::new(Username { username: format!("Username: {}", req.into_inner().username) }))
    }
}

pub async fn run() -> Result<Nil> {
    info!("rpc run");

    let internal_rpc = InternalRpcImpl::default();
    let service = InternalRpcServer::new(internal_rpc);

    let socket = "127.0.0.1:3000".parse()?;
    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(service)
        .serve(socket)
        .await?;

    Ok(nil)
}
