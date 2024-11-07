// use tonic::{transport::Server, Request, Response, Status};
use simpledb::{
    edge_db_client::{EdgeDbClient},
    Data,
};

pub mod simpledb {
    tonic::include_proto!("simpledb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EdgeDbClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Data {
        key: "Zxnia".into(),
        value: "".into(),
    });

    println!("Sending set_data request to gRPC Server...");
    let response = client.set_data(request).await?;

    println!("RESPONSE={:?}", response);

    println!("Sending get_data request to gRPC Server...");

    let request = tonic::Request::new(Data {
        key: "Zxnia".into(),
        value: "".into(),
    });
    let response = client.get_data(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}