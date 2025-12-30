use crate::blockchain::chain::Blockchain;
use crate::storage::file;
use clap::{Parser, Subcommand};

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
                let blockchain = file::load_chain();
                if blockchain.blocks.is_empty() {
                    file::save_chain(Blockchain::init().blocks.as_slice());
                }
            }
            Commands::AddTxt { payload } => {
                println!("add-txt: {}", payload)
            }
            Commands::Mine => println!("mine-block"),
            Commands::Print => {
                println!("print-chain");
                let blockchain = file::load_chain();
                let blocks = blockchain.blocks;
                if !blocks.is_empty() {
                    println!("{}", blocks.last().unwrap().previous_hash);
                }
            }
        }
    }
}
