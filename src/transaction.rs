use std::process::Output;
use log::info;
use serde::{Deserialize, Serialize};

const SUBSIDY: i32 = 10;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TXOutputs{
    pub output: Vec<TXOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction{
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>
}

impl Transaction {
    pub fn new_UTXO(wallet: &Wallet,to: &str,amount: i32,utxo: &UTXOSet) -> Result<Transaction> {
        info!(
            "new UTXO Transaction from: {} to: {}",
            wallet.get_address(),
            to
        );
        let mut vin = Vec::new();
        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);

        let acc_v = utxo.fi
    }

}

