use encfs_cracker::crypto::encfs_config::EncfSConfig;
use std::time::Instant;

fn main() {
    let xml = r#"<config>
    <salt>SGVsbG8=</salt>
    <iterations>1000</iterations>
    <keySize>32</keySize>
    <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
</config>"#;
    let config = EncfSConfig::from_xml(xml).unwrap();
    let password = "password";
    
    let iterations = 10000;
    let start = Instant::now();
    for _ in 0..iterations {
        config.verify_password(password);
    }
    let duration = start.elapsed();
    println!("Time taken for {} verifications: {:?}", iterations, duration);
    println!("Average time per verification: {:?}", duration / iterations);
}
