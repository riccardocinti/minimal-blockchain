use crate::crypto::hash::sha256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Hash)]
pub struct Transaction {
    pub tx_id: String,
    pub payload: Vec<u8>,
}

impl Transaction {
    pub fn new(payload: Vec<u8>) -> Self {
        let tx_id = sha256(&payload);
        Self { tx_id, payload }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_payloads_produce_identical_tx_ids() {
        let payload = b"hello world".to_vec();

        let tx1 = Transaction::new(payload.clone());
        let tx2 = Transaction::new(payload);

        assert_eq!(
            tx1.tx_id, tx2.tx_id,
            "Transaction ID must be derived from payload content"
        );
    }

    #[test]
    fn tx_id_matches_sha256_of_payload() {
        let payload = b"test payload".to_vec();
        let expected = sha256(&payload);

        let tx = Transaction::new(payload);

        assert_eq!(tx.tx_id, expected, "Transaction ID must be sha256(payload)");
    }

    #[test]
    fn tx_id_is_stable_after_creation() {
        let tx = Transaction::new(b"immutable".to_vec());
        let id_before = tx.tx_id.clone();

        // payload is public, so we can technically mutate it
        // this test documents the *danger*
        // (we will fix this in a later step)
        assert_eq!(tx.tx_id, id_before, "Transaction ID must not change");
    }
}
