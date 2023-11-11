use clap::{Args, Subcommand};
use self::pattern_count::PatternCountArgs;
use self::frequent_words::FrequentWordsArgs;
mod pattern_count;
mod frequent_words;

/// Input files accepted as inputs.
// pub enum InputFile
// {
//     Text,
//     Json,
// }

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct UtilArgs 
{
    #[command(subcommand)]
    pub command: Option<UtilCommands>,
}

#[derive(Debug, Subcommand)]
pub enum UtilCommands 
{
    PatternCount(PatternCountArgs),
    FrequentWords(FrequentWordsArgs),
}