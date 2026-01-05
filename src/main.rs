use clap::Parser;
use minimal_blockchain::node::commands::Cli;

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
