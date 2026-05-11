use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(short, long, value_delimiter = ',')]
    fragments: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Config path: {:?}", args.config);
    println!("Fragments: {:?}", args.fragments);
    Ok(())
}
