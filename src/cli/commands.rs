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
            Commands::Init => println!("init-chain"),
            Commands::AddTxt { payload } => {println!("add-txt: {}", payload)}
            Commands::Mine => println!("mine-block"),
            Commands::Print => println!("print-chain"),
        }
    }
}
