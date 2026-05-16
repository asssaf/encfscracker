use encfs_cracker::crypto::encfs_config::EncfSConfig;
use std::time::Instant;

fn main() {
    let xml = r#"<boost_serialization>
    <cfg>
        <saltData>SGVsbG8=</saltData>
        <kdfIterations>1000</kdfIterations>
        <keySize>32</keySize>
        <encodedKeyData>S2V5RGF0YQ==</encodedKeyData>
    </cfg>
</boost_serialization>"#;
    let config = EncfSConfig::from_xml(xml).unwrap();
    let password = "password";

    let iterations = 10000;
    let start = Instant::now();
    for _ in 0..iterations {
        config.verify_password(password);
    }
    let duration = start.elapsed();
    println!(
        "Time taken for {} verifications: {:?}",
        iterations, duration
    );
    println!("Average time per verification: {:?}", duration / iterations);
}
