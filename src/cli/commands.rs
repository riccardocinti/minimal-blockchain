use crate::blockchain::chain::Blockchain;
use crate::blockchain::mempool::Mempool;
use crate::node::node::{Node, NodeConfig};
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
    Start,
    Print,
}

impl Cli {
    pub fn execute(self) {
        match self.command {
            Commands::Start => {
                println!("start-node");
                let chain_file = Path::new(file::CHAIN_FILE);
                match file::load_chain(chain_file) {
                    Ok(chain) => {
                        if chain.blocks.is_empty() {
                            file::save_chain(chain_file, Blockchain::init().blocks.as_slice());
                        }
                        let mut node =
                            Node::new(chain, Mempool::new(), NodeConfig::new(true, 2, 1), 1);
                        node.run().expect("Error running the node");
                    }
                    Err(e) => {
                        println!("Error: {:?}", e)
                    }
                }
            }
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
