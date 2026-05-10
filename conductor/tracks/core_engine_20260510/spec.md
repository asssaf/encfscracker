# Specification - Build core EncfS key cracking engine

## Objective
Implement the fundamental engine for cracking EncfS keys. This includes parsing the EncfS configuration, generating password combinations from fragments, and validating them against the EncfS master key.

## Scope
- EncfS `.encfs6.xml` parser.
- Password combination generator (permutations of fragments).
- Key derivation and validation logic (using PBKDF2, AES/SIV or relevant EncfS standards).
- Basic CLI for providing the config file and fragment list.
- Minimal state persistence for tried combinations (unencrypted for the initial phase).

## Technical Requirements
- **Language:** Rust
- **Libraries:** `clap` (CLI), `serde` & `quick-xml` (XML parsing), `rust-crypto` or `ring` (cryptography).
- **Concurrency:** Use `rayon` or `tokio` for parallel combination testing.

## Verification Criteria
- Correctly identify the password for a known EncfS volume given the correct fragments.
- Successfully parse various EncfS configuration files.
- Demonstrate multi-core utilization during the cracking process.
