use tonic::{transport::Server, Request, Response, Status};
use simpledb::{
    edge_db_server::{EdgeDb, EdgeDbServer},
    Data, Dummy, DataBlock,Certificate,
    edge_replica_server::{EdgeReplica},
    edge_certification_server::{EdgeCertification}
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

#[tonic::async_trait]
impl EdgeReplica for Edge {
    async fn replicate(
        &self,
        request: Request<DataBlock>
    ) -> Result<Response<Dummy>, Status> {
        println!("Received request from: {:?}", request);
        let response = simpledb::Dummy {};
        Ok(Response::new(response))
    }
}

#[tonic::async_trait]
impl EdgeCertification for Edge {
    async fn certify(
        &self,
        request: Request<Certificate>
    ) -> Result<Response<Dummy>, Status> {
        println!("Received Certification request from: {:?}", request);
        let response = simpledb::Dummy {};
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