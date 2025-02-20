use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::server::Server;
use crate::transaction::Transaction;
use crate::utxoset::UTXOSet;
use crate::wallets::{Wallet, Wallets};
use bitcoincash_addr::Address;
use clap::{arg, Command, Subcommand};
use std::process::exit;

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("behrouz.r.fa@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(Command::new("createwallet").about("create a wallet"))
            .subcommand(Command::new("listaddresses").about("list all addresses"))
            .subcommand(Command::new("reindex").about("reindex UTXO"))
            .subcommand(
                Command::new("getbalance")
                    .about("get balance in the blockchain")
                    .arg(arg!(<ADDRESS>"'The Address it get balance for'")),
            )
            .subcommand(
                Command::new("startnode")
                    .about("start the node server")
                    .arg(arg!(<PORT>"'the port server bind to locally'")),
            )
            .subcommand(
                Command::new("create")
                    .about("Create new blockchain")
                    .arg(arg!(<ADDRESS>"'The address to send genesis block reward to'")),
            )
            .subcommand(
                Command::new("send")
                    .about("send in the blockchain")
                    .arg(arg!(<FROM>"'Source wallet address'"))
                    .arg(arg!(<TO>"'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>"'Amount to send'"))
                    .arg(arg!(-m --mine "'Mine immediately'")),
            )
            .subcommand(
                Command::new("startminer")
                    .about("start the miner server")
                    .arg(arg!(<PORT>"'The port server bind to locally'"))
                    .arg(arg!(<ADDRESS>"'Wallet address'")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("startminer") {
            let port = if let Some(port) = matches.get_one::<String>("PORT") {
                port
            } else {
                println!("PORT not supply!: usage");
                exit(1)
            };

            
        }

        Ok(())
    }
}
