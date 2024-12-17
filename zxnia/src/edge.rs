use std::{env, fmt::format};
use clap::Parser;

use tonic::{transport::Server, Request, Response, Status};
use simpledb::{
    edge_db_server::{EdgeDb, EdgeDbServer},
    Data, Dummy, DataBlock,Certificate,
    edge_replica_server::EdgeReplica,
    edge_certification_server::EdgeCertification
};

pub mod simpledb {
    tonic::include_proto!("simpledb");
}

mod config_reader;
use config_reader::{Config,DatabaseConfig,ServerConfig};
mod db;
use db::DbWrapper;

#[derive(Debug, Default)]
pub struct Edge {
    db: DbWrapper,
}

#[tonic::async_trait]
impl EdgeDb for Edge {
    async fn set_data(
        &self,
        request: Request<Data>,
    ) -> Result<Response<Dummy>, Status> {
        println!("Received write request from: {:?}", request);
        let data = request.into_inner();
        
        let _ = self.db.write(&data.key, &data.value);

        let response = simpledb::Dummy{};

        Ok(Response::new(response))
    }

    async fn get_data(
        &self,
        request: Request<Data>
    ) -> Result<Response<Data>, Status> {
        println!("Received request from: {:?}", &request);
        let key = &request.into_inner().key;
        let value = self.db.read(&key);
        

        let response = simpledb::Data {
            key: format!("{}", &key).into(),
            value: value,
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
        println!("Received Certification response from: {:?}", request);
        let response = simpledb::Dummy {};
        Ok(Response::new(response))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server host
    #[arg(short = 'H', long, default_value = "0.0.0.0")]
    host: String,

    /// Server port
    #[arg(short, long, default_value_t = 50051)]
    port: u16,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Starting Edge Replica at {}:{}", args.host, args.port);
    format!("{}:{}", args.host, args.port);
    let addr = "[::1]:50051".parse()?;
    let edge_service = Edge::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(EdgeDbServer::new(edge_service))
        .serve(addr)
        .await?;
    Ok(())
}