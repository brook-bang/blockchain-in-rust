use super::*;
use crate::block::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use sled;
use std::collections::HashMap;
use log::{debug, info};

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";


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
        info!("Creating new blockchain");
        std::fs::remove_dir_all("data/blocks").ok();
        let db = sled::open("data/blocks")?;
        debug!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        
    }

        /// SignTransaction signs inputs of a Transaction
        pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
            let prev_TXs = self.get_prev_TXs(tx)?;
            tx.sign(private_key, prev_TXs)?;
            Ok(())
        }

    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {}
}