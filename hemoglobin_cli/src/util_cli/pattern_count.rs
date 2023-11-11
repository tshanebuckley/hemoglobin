use clap::Args;

#[derive(Debug, Args)]
pub struct PatternCountArgs 
{
    #[arg(short, long)]
    pub text: Option<String>,

    #[arg(short, long)]
    pub pattern: Option<String>,
}

#[derive(Debug, Args)]
pub struct PatternCountTextArgs
{
    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(value_enum, short, long)]
    pub file_type: Option<bool>
}
