use crate::blockchain::chain::Blockchain;
use crate::storage::file;
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    AddTxt { payload: String },
    Mine,
    Print,
}

impl Cli {
    pub fn execute(self) {
        match self.command {
            Commands::Init => {
                println!("init-chain");
                let chain_file = Path::new(file::CHAIN_FILE);
                let blockchain = file::load_chain(chain_file);
                if blockchain.unwrap().blocks.is_empty() {
                    file::save_chain(chain_file, Blockchain::init().blocks.as_slice());
                }
            }
            Commands::AddTxt { payload } => {
                println!("add-txt: {}", payload)
            }
            Commands::Mine => println!("mine-block"),
            Commands::Print => {
                println!("print-chain");
                let blockchain = file::load_chain(Path::new(file::CHAIN_FILE));
                let blocks = blockchain.unwrap().blocks;
                if !blocks.is_empty() {
                    println!("{}", blocks.last().unwrap().previous_hash);
                }
            }
        }
    }
}
