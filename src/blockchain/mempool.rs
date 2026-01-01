use crate::blockchain::transaction::Transaction;

pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Mempool {
        Mempool {
            transactions: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn drain(&mut self) -> Vec<Transaction> {
        std::mem::take(&mut self.transactions)
    }

    pub fn peek(&self) -> Vec<Transaction>  {
        self.transactions.clone()
    }

}
