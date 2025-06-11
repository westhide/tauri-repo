mod hello_world {
    tonic::include_proto!("helloworld");
}

use crate::log::{info, instrument};
use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("rpc run");

    let addr = "127.0.0.1:3000".parse()?;
    let service = GreeterServer::new(MyGreeter::default());

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::new())
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
