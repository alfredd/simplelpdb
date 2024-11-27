
use std::fmt::format;

use tonic::Response;
use warp::{reject, Filter, Rejection, Reply};

mod zx_data_structures;
mod client;
use zx_data_structures::{Transaction, WriteKeyRequest};
use client::EdgeClient;


static EDGE_CLIENT_ADDR : &'static str = "http://[::1]:50051";

async fn read_handler(name: String) -> Result<impl Reply, Rejection> {
    println!("Executing code from a handler function.");
    let client = EdgeClient::new(EDGE_CLIENT_ADDR.into());
    if let Ok(result) = client.read_key(name.clone()).await {
        println!("Result is {}", result);
        Ok(format!("Hello, {}!", result))
    } else {
        println!("Error");
        Err(reject::not_found())
    }
    
}

async fn write_handler(wr: WriteKeyRequest) -> Result<impl Reply, Rejection> {
    let response = format!("Write value for k={}, v={}", wr.k, wr.v);
    println!("Request received: {}", response);
    let client = EdgeClient::new(EDGE_CLIENT_ADDR.into());
    match client.write_key(wr).await {
        Ok(_) => return Ok(format!("Result written successfully: {}", response )),
        Err(_) => {
            println!("Error when trying to write.");
            return Err(reject::not_found());
        }
    }
}

async fn txn_handler(txn: Transaction) -> Result<impl Reply, Rejection> {
    let response = format!("Txn  {{edge:{:?}, cloud:{:?} }}", txn.edge, txn.cloud );
    Ok(warp::reply::html(response))
}

async fn read_all_handler() -> Result<impl Reply, Rejection> {
    let response = format!("Received read all.");
    Ok(warp::reply::html(response))
}

#[tokio::main]
async fn main() {

    // Requried interfaces

    // 1. GET /zx/readall
    //      get all data from edge and cloud
    //      response {edge:{}, cloud{}}

    let readall = warp::path("readall")
        .and_then(read_all_handler);
    // .map(|| {
    //     let data = vec![1, 2, 3, 4];
    //     warp::reply::json(&data)
    // });
    // 2. GET /zx/read/KEY
    //      read key k
    let read_key = warp::get()
        .and(warp::path("read"))
        .and(warp::path::param::<String>())
        .and_then(read_handler);

    // 3. PUT /zx/write?k=KEY&v=VALUE
    //      write key k

    let write_key = warp::get()
        .and(warp::path("write"))
        .and(warp::query::<WriteKeyRequest>())
        .and_then(write_handler);

    // 4. POST /zx/txn/ {edge: {a:'bd', b:'sd', c:45}, cloud: {ca:43}}
    let post_txn = warp::post()
        .and(warp::path("txn"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(txn_handler);
        // .map(|txn: Transaction| warp::reply::json(&txn));

    // edge_client::client_call();

    // Setup Routes
    let routes = warp::get()
        .and(readall)
        .or(read_key)
        .or(write_key)
        .or(post_txn);
    warp::serve(routes).run(([127, 0, 0, 1], 50052)).await;
}
