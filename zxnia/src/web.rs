

use warp::http::Response;
use warp::Filter;
use std::collections::HashMap;



mod zx_data_structures;
use zx_data_structures::{WriteKeyRequest, Transaction};


#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path!("hello" / String))
        .map(|name| format!("Hello, {}!", name));

    // Requried interfaces

    // 1. GET /zx/readall
    //      get all data from edge and cloud
    //      response {edge:{}, cloud{}}

    let readall = warp::path("readall").map(|| {
        let data = vec![1, 2, 3, 4];
        warp::reply::json(&data)
    });
    // 2. GET /zx/read?k=PARAM
    //      read key k

    let read_key = warp::get()
        .and(warp::path("read"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("k") {
            Some(key) => Response::builder().body(format!("Key = {}", key)),
            None => Response::builder().body(String::from("No 'key' in query")),
        });

    // 3. PUT /zx/write?k=KEY&v=VALUE
    //      write key k

    let write_key = warp::get()
        .and(warp::path("write"))
        .and(warp::query::<WriteKeyRequest>())
        .map(|p: WriteKeyRequest|  {
            Response::builder().body(format!("Key={}, Value={}", p.k, p.v))
        });


    // 4. POST /zx/txn/ {edge: {a:'bd', b:'sd', c:45}, cloud: {ca:43}}
    let post_txn = warp::post()
        .and(warp::path("txn"))
        .and(warp::body::content_length_limit(1024*16))
        .and(warp::body::json())
        .map(|txn: Transaction|{
            warp::reply::json(&txn)
        });
    
        // edge_client::client_call();

    // Setup Routes
    let routes = warp::get()
        .and(hello)
        .or(readall)
        .or(read_key)
        .or(write_key)
        .or(post_txn);
    warp::serve(routes).run(([127, 0, 0, 1], 50052)).await;
}
