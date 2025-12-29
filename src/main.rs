use clap::Parser;
use minimal_blockchain::cli::commands::Cli;

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
