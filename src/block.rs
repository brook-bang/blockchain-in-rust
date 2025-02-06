use crate::transaction::Transaction;

const TARGET_HEXS: usize = 4;

pub struct Block{
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
    height: i32,
}