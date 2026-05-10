# Tech Stack - EncfS Key Cracker

## Core Language & Runtime
- **Rust:** Chosen for its industry-leading memory safety, zero-cost abstractions, and high-performance capabilities, making it ideal for a secure and efficient cracking tool.

## Frameworks & Libraries
- **CLI Framework:** `clap` - The standard crate for building robust and user-friendly command-line interfaces in Rust, providing powerful argument parsing and help generation.
- **Cryptography:** `RustCrypto` - A collection of pure-Rust, secure, and audited cryptographic algorithms for handling EncfS key derivation and encryption at rest.
- **State Management:** `sled` - A high-performance, embedded key-value store (modern successor to LevelDB) used for persistent tracking of tried password combinations.
- **EncfS Integration:** Custom implementation using `RustCrypto` to parse `.encfs6.xml` files and perform key derivation checks.

## Data Storage
- **Progress Tracking:** `sled` database for efficient, atomic updates of progress and tried combinations.
- **Sensitive Data:** Custom encrypted file formats for fragment lists and recovery logs, secured using `RustCrypto`.

## Development & Tooling
- **Build System:** `cargo` - Rust's integrated build tool and package manager.
- **Testing:** Rust's built-in testing framework for unit and integration tests.
- **Linter:** `clippy` for ensuring idiomatic and high-quality Rust code.
