use tonic::{transport::Server, Request, Response, Status};
use simpledb::{
    edge_db_server::{EdgeDb, EdgeDbServer},
    Data, Dummy,
};

pub mod simpledb {
    tonic::include_proto!("simpledb");
}

#[derive(Debug, Default)]
pub struct Edge {}

#[tonic::async_trait]
impl EdgeDb for Edge {
    async fn set_data(
        &self,
        request: Request<Data>,
    ) -> Result<Response<Dummy>, Status> {
        println!("Received request from: {:?}", request);
        let response = simpledb::Dummy{};

        Ok(Response::new(response))
    }

    async fn get_data(
        &self,
        request: Request<Data>
    ) -> Result<Response<Data>, Status> {
        println!("Received request from: {:?}", request);
        let response = simpledb::Data {
            key: format!("{}", request.into_inner().key).into(),
            value: format!("Hello {}!", "Test Data Value").into(),
        };
        Ok(Response::new(response))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let edge_service = Edge::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(EdgeDbServer::new(edge_service))
        .serve(addr)
        .await?;

    Ok(())
}