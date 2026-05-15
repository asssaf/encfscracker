# Implementation Plan: Encryption at Rest

## Objective
Implement authenticated encryption (AES-256-GCM) for the persistent state database (Sled) to secure sensitive fragment data and progress logs. Use Argon2id for key derivation from a user-provided master password.

## Key Files & Context
- `src/state/sled_db.rs`: Primary file for state persistence logic.
- `src/main.rs`: CLI entry point for password prompting and initialization.
- `Cargo.toml`: Add dependencies for `argon2`, `rand`, and `rpassword`.

## Phase 1: Cryptographic Infrastructure & KDF
**Goal:** Implement the core cryptographic logic for key derivation and data encryption.

- [x] Task: Add dependencies to `Cargo.toml` (`argon2`, `rand`, `rpassword`). 47fad33
- [ ] Task: Implement `CryptoManager` module in `src/crypto/mod.rs` (or `src/crypto/state_encryption.rs`).
    - [ ] Task: TDD - Write tests for Argon2id key derivation.
    - [ ] Task: Implement `derive_key(password: &str, salt: &[u8]) -> [u8; 32]`.
    - [ ] Task: TDD - Write tests for AES-256-GCM encryption/decryption.
    - [ ] Task: Implement `encrypt(data: &[u8], key: &[u8]) -> Vec<u8>` and `decrypt(data: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>>`.
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Encrypted State Management (Sled Integration)
**Goal:** Refactor `SledDb` to use the `CryptoManager` for all sensitive storage operations.

- [ ] Task: Add `config` tree to `SledDb` constants.
- [ ] Task: TDD - Write integration tests for `SledDb` with encryption (using a mock key).
- [ ] Task: Update `SledDb` struct to include a thread-safe storage for the `master_key: OnceCell<[u8; 32]>`.
- [ ] Task: Implement `SledDb::initialize_encryption(&self, password: &str)` for new databases (generate salt, derive key, save salt).
- [ ] Task: Implement `SledDb::unlock(&self, password: &str)` for existing databases (load salt, derive key, verify with canary).
- [ ] Task: Refactor `SledDb` methods to encrypt/decrypt data:
    - [ ] Task: `add_fragment` and `list_fragments`.
    - [ ] Task: `mark_as_tried` and `is_tried` (note: keys in `tried_combinations` tree must also be hashed or encrypted).
    - [ ] Task: `save_checkpoint` and `load_checkpoint`.
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: CLI Integration & UX
**Goal:** Implement the user-facing password prompt and secure initialization flow.

- [ ] Task: TDD - Write tests for the password prompt logic (mocking stdin).
- [ ] Task: Implement a secure password prompt using `rpassword` in `src/main.rs`.
- [ ] Task: Update the "setup" flow: if no DB exists, prompt for a new master password.
- [ ] Task: Update the startup flow: if a DB exists, prompt for the master password before any operations.
- [ ] Task: Add clear error handling for incorrect passwords.
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)

## Phase 4: Final Verification & Documentation
**Goal:** Ensure security guarantees are met and documented.

- [ ] Task: Verify that sensitive data is not stored in plaintext using a hex editor or Sled-inspect tool.
- [ ] Task: Update `README.md` or technical documentation to include the new security model.
- [ ] Task: Conductor - User Manual Verification 'Phase 4' (Protocol in workflow.md)

## Verification & Testing
- **Automated Tests:** Unit tests for `CryptoManager` and integration tests for `SledDb` encryption.
- **Manual Verification:** 
    1. Create a new database with a password.
    2. Add fragments.
    3. Close and reopen the database with the correct password (verify fragments are there).
    4. Attempt to open with the wrong password (verify failure).
    5. Inspect the `.db` files to ensure no plaintext "fragments" or "passwords" are visible.
