use encfs_cracker::fragment_combination::parallel::parallel_combination_test;
use encfs_cracker::state::sled_db::SledDb;
use std::time::Instant;
use tempfile::tempdir;

fn main() -> anyhow::Result<()> {
    let fragments: Vec<String> = (0..100).map(|i| i.to_string()).collect();
    let k = 2; // 100P2 = 9900 combinations
    
    // Benchmark without state
    {
        let start = Instant::now();
        parallel_combination_test(&fragments, k, |_| {
            // Simulate work
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            std::hint::black_box(sum);
            false
        });
        let duration = start.elapsed();
        println!("Stateless duration: {:?}", duration);
    }
    
    // Benchmark with state
    {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("bench_db");
        SledDb::init(&db_path)?;
        
        let start = Instant::now();
        parallel_combination_test(&fragments, k, |_| {
             // Simulate work
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            std::hint::black_box(sum);
            false
        });
        let duration = start.elapsed();
        println!("Persistent state duration: {:?}", duration);
    }
    
    Ok(())
}
