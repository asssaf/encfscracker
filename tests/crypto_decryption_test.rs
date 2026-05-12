use encfs_cracker::crypto::decrypt_encoded_key_data;
use aes::Aes256;
use cfb_mode::{Decryptor, Encryptor};
use aes::cipher::{KeyIvInit, AsyncStreamCipher};

type Aes256CfbDec = Decryptor<Aes256>;
type Aes256CfbEnc = Encryptor<Aes256>;

#[test]
fn test_aes_decryption_with_valid_key() {
    let key = b"0123456789abcdef0123456789abcdef"; // 32 bytes
    let iv = vec![0u8; 16];
    let plaintext = b"test plaintext data";
    
    // Encrypt data to get valid ciphertext
    let mut ciphertext = plaintext.to_vec();
    Aes256CfbEnc::new(key.into(), iv.as_slice().into()).encrypt(&mut ciphertext);
    
    let result = decrypt_encoded_key_data(key, &iv, &ciphertext).unwrap();
    assert_eq!(result, plaintext);
}
