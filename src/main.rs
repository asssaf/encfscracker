use clap::Parser;
use std::path::PathBuf;
use encfs_cracker::config::CrackerConfig;
use encfs_cracker::crypto::encfs_config::EncfSConfig;
use encfs_cracker::orchestration::parallel::ParallelCracker;

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
    
    // Initialize state database
    let db_path = PathBuf::from("cracker_state.db");
    
    if args.reset_state {
        let db = encfs_cracker::state::sled_db::SledDb::open(&db_path)?;
        db.reset_state()?;
        println!("State reset successfully.");
    }

    let xml = std::fs::read_to_string(args.config)?;
    let encfs_config = EncfSConfig::from_xml(&xml)?;
    
    let config = CrackerConfig {
        fragments: args.fragments,
        encfs_config,
        db_path,
    };
    
    let cracker = ParallelCracker::new(config)?;
    if let Some(password) = cracker.run()? {
        println!("Password found: {}", password);
        std::fs::write("recovered_password.txt", &password)?;
    } else {
        println!("No password found.");
    }

    Ok(())
}
