use anyhow::Result;
use clap::{Parser, Subcommand};
use rox::lexer::Lexer;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tokenize { filename: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(filename).expect("Failed to read file");

            for token in Lexer::new(&file_contents) {
                match token {
                    Ok(token) => println!("{token}"),
                    Err(e) => println!("Lexing error: {}", e),
                }
            }
            println!("EOF null");
        }
    }

    Ok(())
}
