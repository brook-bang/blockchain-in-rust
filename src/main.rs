use cli::Cli;

use crate::errors::Result;

mod block;
mod cli;
mod errors;
mod blockchain;
mod transaction;
mod wallets;
mod tx;
mod utxoset;
mod server;

fn main() -> Result<()>{
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
