use bitcoincash_addr::Address;
use log::{debug, info};
use crate::transaction::Transaction;
use super::*;

#[derive(Debug)]
pub struct Blockchain {
    pub tip: String,
    pub db:sled::Db,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");

        let db = sled::open("data/block")?;
        let hash = match db.get("LAST")? {
            Some(l) => l.to_vec(),
            None => Vec::new(),
        };

        info!("Found block database");

        let lasthash = if hash.is_empty() {
            String::new()
        } else {
            String::from_utf8(hash.to_vec())?
        };

        Ok(Blockchain {tip: lasthash, db})
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Create new blockchain");
        std::fs::remove_dir("data/blocks").ok();
        let db =sled::open("data/blocks")?;
        debug!("Creating new block database");
        let cbtx = Transaction::n


    }

        /// SignTransaction signs inputs of a Transaction
        pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
            let prev_TXs = self.get_prev_TXs(tx)?;
            tx.sign(private_key, prev_TXs)?;
            Ok(())
        }

    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {}
}