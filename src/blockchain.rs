use super::*;
use crate::block::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use sled;
use std::collections::HashMap;
use std::hash::Hash;
use log::{debug, info};

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Debug)]
pub struct Blockchain {
    pub tip: String,
    pub db: sled::Db,
}

pub struct BlockchainIterator<'a> {
    current_hash: String,
    bc:&'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");
        let db = sled::open("data/blocks")?;
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
        Ok(Blockchain{tip: lasthash,db})
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");
        std::fs::remove_dir_all("data/blocks").ok();
        let db = sled::open("data/blocks")?;
        debug!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesisi: Block = Block::new_genesis_block(cbtx);
        db.insert(genesisi.get_hash(), serialize(&genesisi)?)?;
        db.insert("LAST", genesisi.get_hash().as_bytes())?;
        let bc = Blockchain {
            tip: genesisi.get_hash(),
            db
        };
        bc.db.flush()?;
        Ok(bc)
    }

    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            bc: &self,
        }
    }

    pub fn find_UTXO(&self) -> HashMap<String,TXOutputs> {
        let mut utxos: HashMap<String,TXOutputs> = HashMap::new();
        let mut spend_txos: HashMap<String,Vec<i32>> = HashMap::new();
        for block in self.iter() {
            for tx in block.get_transaction(){
                for index in 0..tx.vout.len() {
                    
                }
            }
        }
    }



    pub fn mine_block(&mut self,transactions: Vec<Transaction>) -> Result<Block> {
        info!("mine a new block");
        for tx in &transactions {
            if !self.verify_transaction(tx)? {
                return Err(format_err!("ERROR: Invalid transaction"));
            }
        }

        let lasthash = self.db.get("LAST")?.unwrap();
        let newblock = Block::new_block(
            transactions,
            String::from_utf8(lasthash.to_vec())?,
            self.get_best_height()? + 1,
        )?;

        self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
        self.db.insert("LAST", newblock.get_hash().as_bytes())?;
        self.db.flush()?;
        self.tip = newblock.get_hash();
        Ok(newblock)
    }

    pub fn find_transaction(&self,id: &str) -> Result<Transaction> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        Err(format_err!("Transaction is not found"))
    }

    fn get_prev_TXs(&self,tx: &Transaction) -> Result<HashMap<String,Transaction>> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.vin {
            let prev_TX = self.find_transaction(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }

    pub fn verify_transaction(&self,tx: &Transaction) -> Result<bool>{
        if tx.is_coinbase() {
            return Ok(true);
        }
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.verify(prev_TXs)
    }

    pub fn get_best_height(&self) -> Result<i32> {
        let lasthash = if let Some(h) = self.db.get("LAST")? {
            h
        } else {
            return Ok(-1);
        };
        let last_data = self.db.get(lasthash)?.unwrap();
        let last_block: Block = deserialize(&last_data.to_vec())?;
        Ok(last_block.get_height())
    }


}


impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
