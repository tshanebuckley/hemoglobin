use clap::Args;

#[derive(Debug, Args)]
pub struct FrequentWordsArgs 
{
    #[arg(short, long)]
    pub text: Option<String>,

    #[arg(short, long)]
    pub word_size: Option<u64>,
}