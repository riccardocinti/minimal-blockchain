use serde::{Serialize, Deserialize};
use crate::crypto::hash::sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub payload: Vec<u8>,
}

impl Transaction {
    pub fn new(payload: Vec<u8>) -> Self {
        let tx_id = sha256(&payload);
        Self {tx_id, payload}
    }
}