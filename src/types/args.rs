use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None )]
pub struct Args {
    #[arg(short = 'c', long, default_value = "git-commit")]
    pub config: String,

    #[arg(short = 'd', long)]
    pub description: Option<String>,
}
