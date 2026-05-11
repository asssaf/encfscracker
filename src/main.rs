use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(short, long, value_delimiter = ',')]
    fragments: Vec<String>,

    #[arg(long, default_value_t = false)]
    reset_state: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Config path: {:?}", args.config);

    if args.reset_state {
        if let Some(db) = encfs_cracker::state::sled_db::SledDb::get() {
            db.reset_state()?;
            println!("State reset successfully.");
        }
    }

    Ok(())
}
