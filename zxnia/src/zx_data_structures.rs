
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct WriteKeyRequest {
    pub k: String,
    pub v: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReadRequest {
    pub k: String
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub edge: HashMap<String, String>,
    pub cloud: HashMap<String, String>,
}