use crate::cli::Cli;
use crate::errors::Result;

mod block;

mod errors;
mod blockchain;
mod transaction;
mod wallets;
mod tx;
mod utxoset;
mod server;

fn main() {
    println!("Hello, world!");
}
