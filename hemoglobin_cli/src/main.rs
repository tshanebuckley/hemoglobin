use clap::{Parser, Subcommand};
use hemoglobin_core::util::{pattern_count, frequent_words};
use util_cli::{UtilArgs, UtilCommands};
mod util_cli;

/// A cli tool for running bioinformatics algorithms implemented in rust.
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "hemoglobin")]
#[command(about = "A cli tool for running bioinformatics algorithms implemented in rust.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands 
{
    Util(UtilArgs),
}

fn main() 
{
    let args = Cli::parse();

    match args.command {
        Commands::Util(util_args) => {
            let util_command = util_args.command.unwrap();
            match util_command {
                UtilCommands::PatternCount(util) => {
                    let text = &util.text.unwrap();
                    let result = pattern_count(text, &util.pattern.unwrap());
                    println!("{result:?}");
                },
                UtilCommands::FrequentWords(util) => {
                    let text = &util.text.unwrap();
                    let result = frequent_words(text, util.word_size.unwrap());
                    for word in result
                    {
                        print!("{word} ");
                    }
                    println!();
                }
            }
        }
    }
}
