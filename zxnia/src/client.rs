use core::fmt;
use std::future::{Future, IntoFuture};

use serde::de::value;
// use tonic::{transport::Server, Request, Response, Status};
use simpledb::{
    edge_db_client::{EdgeDbClient},
    Data,
};

mod zx_data_structures;
use tonic::{client, transport::Channel};
use zx_data_structures::{Transaction, WriteKeyRequest};

mod simpledb {
    tonic::include_proto!("simpledb");
}

pub struct EdgeClient {
    EDGE_CLIENT_ADDRESS: String
}

impl EdgeClient {
    pub fn new() -> EdgeClient {
        EdgeClient {
            EDGE_CLIENT_ADDRESS: "http://[::1]:50051".into(),
        }
    }
    async fn get_client(&self) -> Result<EdgeDbClient<Channel>, Box<dyn std::error::Error>> {
        let client = EdgeDbClient::connect(self.EDGE_CLIENT_ADDRESS.clone()).await?;
        Ok(client)
    }

    pub fn commit_txn(txn: Transaction) {
    
    }

    fn generate_empty_data_request_object(key: String, value:String) -> tonic::Request<Data> {
        let request = tonic::Request::new( Data{
            key: key,
            value: value,
        });
        request
    }
    
    pub async fn read_key(&self, key:String) -> Result<String, Box<dyn std::error::Error>>   {
        let mut client = self.get_client().await?;
        let request = EdgeClient::generate_empty_data_request_object(key, "".into());

        let response = client.get_data(request).await?;
        let result = response.into_inner().value;
        Ok(result)
        
    }
    
    pub async fn write_key(&self, wr: WriteKeyRequest) -> Result<(), Box<dyn std::error::Error>> {
        let mut client =self.get_client().await?;
        let request = EdgeClient::generate_empty_data_request_object(wr.k, wr.v);
        let response = client.set_data(request).await?;
        println!("Response {:?}", response);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = EdgeDbClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(Data {
    //     key: "Zxnia".into(),
    //     value: "".into(),
    // });

    // println!("Sending set_data request to gRPC Server...");
    // let response = client.set_data(request).await?;

    // println!("RESPONSE={:?}", response);

    // println!("Sending get_data request to gRPC Server...");

    // let request = tonic::Request::new(Data {
    //     key: "Zxnia".into(),
    //     value: "".into(),
    // });
    // let response = client.get_data(request).await?;

    // println!("RESPONSE={:?}", response);
    let eclient = EdgeClient::new();
    eclient.write_key(WriteKeyRequest{
        k:"Test".into(),
        v: "B".into(),
    }).await?;

    let response = eclient.read_key("Test".into()).await?;

    println!("Response of Read: {:?}", response);

    Ok(())
}