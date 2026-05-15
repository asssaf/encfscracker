use clap::Parser;
use std::path::PathBuf;
use encfs_cracker::config::CrackerConfig;
use encfs_cracker::crypto::encfs_config::EncfSConfig;
use encfs_cracker::orchestration::parallel::ParallelCracker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(short, long, value_delimiter = ',')]
    fragments: Vec<String>,

    #[arg(long, default_value_t = false)]
    reset_state: bool,

    #[arg(long, default_value = "cracker_state.db")]
    db_path: PathBuf,

    #[arg(long)]
    add_fragment: Option<String>,

    #[arg(long)]
    group: Option<String>,

    #[arg(long)]
    import_file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    list_fragments: bool,

    #[arg(long, default_value_t = false)]
    clear_fragments: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Initialize state database
    let db_path = args.db_path.clone();
    let db = encfs_cracker::state::sled_db::SledDb::open(&db_path)?;
    
    if args.reset_state {
        db.reset_state()?;
        println!("State reset successfully.");
    }

    if let Some(text) = args.add_fragment {
        let fragment = encfs_cracker::state::Fragment {
            text,
            group_id: args.group,
        };
        db.add_fragment(&fragment)?;
        println!("Fragment added successfully.");
        return Ok(());
    }

    if let Some(path) = args.import_file {
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                let fragment = encfs_cracker::state::Fragment {
                    text: line.to_string(),
                    group_id: args.group.clone(),
                };
                db.add_fragment(&fragment)?;
            }
        }
        println!("Fragments imported successfully.");
        return Ok(());
    }

    if args.list_fragments {
        let fragments = db.list_fragments()?;
        if fragments.is_empty() {
            println!("No fragments found.");
        } else {
            println!("Fragments:");
            for f in fragments {
                let group = f.group_id.unwrap_or_else(|| "none".to_string());
                println!("  - {}: (group: {})", f.text, group);
            }
        }
        return Ok(());
    }

    if args.clear_fragments {
        db.clear_fragments()?;
        println!("All fragments cleared.");
        return Ok(());
    }

    // Default behavior: Crack
    let config_path = args.config.ok_or_else(|| anyhow::anyhow!("Config file is required for cracking"))?;
    let xml = std::fs::read_to_string(config_path)?;
    let encfs_config = EncfSConfig::from_xml(&xml)?;
    
    // Merge command line fragments with DB fragments
    let mut fragments: Vec<encfs_cracker::state::Fragment> = args.fragments
        .into_iter()
        .map(|text| encfs_cracker::state::Fragment { text, group_id: None })
        .collect();
    let db_fragments = db.list_fragments()?;
    fragments.extend(db_fragments);
    drop(db);
    
    let config = CrackerConfig {
        fragments,
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
