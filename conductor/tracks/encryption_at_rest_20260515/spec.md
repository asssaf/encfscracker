# Specification: Encryption at Rest

## Overview
This track implements authenticated encryption for the persistent state database (Sled). It ensures that sensitive password fragments, the log of tried combinations, and progress metadata are protected against unauthorized access when the tool is not running.

## Goals
- Protect all sensitive data stored in the `sled` database.
- Use industry-standard, secure cryptographic primitives.
- Provide a seamless but secure user experience for unlocking the state.

## Functional Requirements

### 1. Secure State Initialization
- When creating a new database, the tool must prompt the user for a master password.
- The master password must be used to derive an encryption key using **Argon2id**.
- A random 16-byte salt must be generated and stored as plaintext within a dedicated `config` tree in the Sled database.

### 2. Database Unlocking
- On every startup, if an existing database is detected, the tool must securely prompt the user for the master password via the CLI.
- The tool must derive the key and attempt to decrypt a small "canary" value or the configuration header to verify the password.
- **Failure Mode:** If the password is incorrect, the tool must display an error and exit immediately.

### 3. Transparent Encryption/Decryption
- All data written to the `fragments`, `tried_combinations`, and `metadata` trees must be encrypted using **AES-256-GCM**.
- Data must be decrypted on-the-fly when read from the database.
- Nonces for AES-GCM should be managed either per-entry or globally, ensuring no nonce reuse.

### 4. Storage of Encryption Metadata
- The Argon2id salt and any global nonces/parameters must be stored in plaintext within a `config` or `header` tree in the Sled database to ensure the database remains self-contained.

## Non-Functional Requirements
- **Performance:** Argon2id parameters should be tuned to provide a balance between security (resistance to GPU cracking) and startup latency (target < 500ms).
- **Security:** Ensure no sensitive data (including the master password) is logged or remains in memory longer than necessary.

## Acceptance Criteria
- [ ] A new state database cannot be created without a master password.
- [ ] An existing database cannot be opened without the correct master password.
- [ ] Attempting to read the `.db` files directly reveals no plaintext fragments or combination logs.
- [ ] The tool exits with a clear error if the wrong password is provided.
- [ ] All existing functionality (add fragment, crack, resume) works identically once the database is unlocked.

## Out of Scope
- Support for changing the master password of an existing database.
- Encrypting the EncfS configuration file itself (it is already encrypted/handled by its own logic).
- Multiple user support or shared keys.
