# Specification - Actual EncfS Key Verification

## Overview
This track involves replacing the current placeholder key verification logic in `src/crypto/encfs_config.rs` with actual EncfS v6 compliant verification. Specifically, it will implement a "Magic Header Check" to determine if a derived key correctly decrypts the EncfS volume key data.

## Functional Requirements
- **AES Decryption:** Implement decryption of `encodedKeyData` using the key derived from the candidate password.
- **Magic Header Validation:** After decryption, check if the resulting data starts with the standard EncfS v6 magic header.
- **Error Handling:** Gracefully handle decryption failures (e.g., incorrect key length or padding issues if applicable).
- **EncfS v6 Compatibility:** Ensure the logic aligns with the standard EncfS v6 configuration format as parsed from `.encfs6.xml`.

## Non-Functional Requirements
- **Performance:** The verification should be highly optimized as it will be called millions of times during the cracking process.
- **Security:** Ensure that sensitive derived keys are not leaked in logs or error messages.

## Acceptance Criteria
- [ ] `EncfSConfig::verify_password` correctly returns `true` for a known valid password and `false` for invalid ones.
- [ ] The implementation uses the `RustCrypto` crates as specified in the Tech Stack.
- [ ] Unit tests are added to verify the magic header check with simulated valid and invalid data.

## Out of Scope
- Full HMAC/Checksum verification of the volume key.
- Supporting multiple EncfS versions beyond v6 (unless already supported by the parser).
- Modifying the orchestration or combination generation logic.
