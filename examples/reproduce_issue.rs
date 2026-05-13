use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2;
use aes::Aes256;
use cfb_mode::Decryptor;
use aes::cipher::{KeyIvInit, AsyncStreamCipher};

type HmacSha1 = Hmac<Sha1>;
type Aes256CfbDec = Decryptor<Aes256>;

fn derive_key(password: &str, salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut derived_key = vec![0u8; 32 + 16];
    pbkdf2::<HmacSha1>(password.as_bytes(), salt, iterations, &mut derived_key);
    derived_key
}

fn set_ivec(master_key: &[u8], master_iv: &[u8], seed: u64) -> [u8; 16] {
    let mut mac = HmacSha1::new_from_slice(master_key).unwrap();
    mac.update(master_iv);
    let mut seed_bytes = [0u8; 8];
    for i in 0..8 {
        seed_bytes[i] = (seed >> (i * 8)) as u8;
    }
    mac.update(&seed_bytes);
    let result = mac.finalize().into_bytes();
    let mut ivec = [0u8; 16];
    ivec.copy_from_slice(&result[0..16]);
    ivec
}

fn unshuffle_bytes(buf: &mut [u8]) {
    for i in (1..buf.len()).rev() {
        buf[i] ^= buf[i - 1];
    }
}

fn flip_bytes(buf: &mut [u8]) {
    buf.reverse();
}

fn calculate_mac(master_key: &[u8], data: &[u8]) -> u32 {
    let mut mac = HmacSha1::new_from_slice(master_key).expect("HMAC can take key of any size");
    mac.update(data);
    let result = mac.finalize().into_bytes();
    
    let mut h = [0u8; 8];
    for i in 0..19 {
        h[i % 8] ^= result[i];
    }
    
    let mut mac64: u64 = 0;
    for &byte in &h {
        mac64 = (mac64 << 8) | (byte as u64);
    }
    ((mac64 >> 32) as u32) ^ (mac64 as u32)
}

fn verify(password: &str, salt_b64: &str, data_b64: &str, iterations: u32) -> bool {
    let salt = general_purpose::STANDARD.decode(salt_b64.replace(|c: char| c.is_whitespace(), "")).unwrap();
    let data = general_purpose::STANDARD.decode(data_b64.replace(|c: char| c.is_whitespace(), "")).unwrap();
    
    let master_key_iv = derive_key(password, &salt, iterations);
    let master_key = &master_key_iv[0..32];
    let master_iv = &master_key_iv[32..48];

    let mut buf = data[4..52].to_vec();
    let checksum = u32::from_be_bytes(data[0..4].try_into().unwrap());
    
    // Pass 1
    let ivec1 = set_ivec(master_key, master_iv, (checksum as u64) + 1);
    Aes256CfbDec::new(master_key.into(), ivec1.as_slice().into()).decrypt(&mut buf);
    unshuffle_bytes(&mut buf);
    flip_bytes(&mut buf);

    // Pass 2
    let ivec2 = set_ivec(master_key, master_iv, checksum as u64);
    Aes256CfbDec::new(master_key.into(), ivec2.as_slice().into()).decrypt(&mut buf);
    unshuffle_bytes(&mut buf);
    
    let calculated_checksum = calculate_mac(master_key, &buf);
    
    println!("  Stored checksum: {:08x}", checksum);
    println!("  Calculated checksum: {:08x}", calculated_checksum);
    
    checksum == calculated_checksum
}

fn main() {
    println!("Case 1 (123456):");
    verify("123456", 
        "8uwRcAXFa76Dvu5mXdNEFUBS4cQ=", 
        "elnyw1B96uMzdGh5CKTXG44EZ25ztz3CvP7tNtsaxbjvqZHwWErHAcZ8SBTiLz4RswtCSw==", 
        3259949);

    println!("\nCase 2 (1234567):");
    verify("1234567", 
        "mdhQKoGaHnOwTMP0GqG1H+mnUpo=", 
        "stVUSAzIfViRRcq+617h0dA/PX1glxBR3ywq5t4dYEgCOwYfgpk447Exfj/eCMtf78YNMw==", 
        4975148);
}
