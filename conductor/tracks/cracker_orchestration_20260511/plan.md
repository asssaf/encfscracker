# Implementation Plan - Main Cracker Orchestration

## Phase 1: Foundation & EncfS Verification [checkpoint: 26e6917]
Goal: Establish the core verification logic and prepare the integration points.

- [x] Task: Implement EncfS configuration parsing and key derivation logic 45f7f6d
    - **Implementation Notes:**
        - Create `src/crypto/encfs_config.rs`.
        - Use `roxmltree` to parse `.encfs6.xml`. Focus on extracting `<salt>`, `<iterations>`, `<keySize>`, and `<encodedKeyData>`.
        - Use `base64` crate to decode the salt and key data.
        - Implement `verify_password(password: &str, config: &EncfSConfig) -> bool`.
        - The logic involves:
            1. Deriving a master key from the password, salt, and iterations using PBKDF2-HMAC-SHA256 (already partially in `src/crypto/mod.rs`).
            2. Using the derived master key to decrypt `encodedKeyData` (usually AES).
            3. Verifying the integrity of the decrypted data (EncfS uses a specific checksum/format for the header).
- [x] Task: Create a unified `CrackerConfig` struct to hold fragments, config, and state 45f7f6d
    - **Implementation Notes:**
        - Define `struct CrackerConfig` in `src/config/mod.rs`.
        - Include `fragments: Vec<String>`, `encfs_config: EncfSConfig`, `db_path: PathBuf`.
        - Add a method to load these from `clap` arguments.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation & EncfS Verification' (Protocol in workflow.md) 45f7f6d

## Phase 2: Sequential Orchestration
Goal: Implement a single-threaded version of the cracker that integrates all components.

- [x] Task: Implement `SequentialCracker` that iterates through combinations and verifies them 9391edd
    - **Implementation Notes:**
        - Create `src/orchestration/sequential.rs`.
        - Use a loop for `k` from 1 to `fragments.len()`.
        - Call `generate_combinations(fragments, k)` from `src/fragment_combination/mod.rs`.
        - Join the combination into a string and call `verify_password`.
- [x] Task: Integrate `SledDb` into `SequentialCracker` for persistence (individual checks) 9391edd
    - **Implementation Notes:**
        - Before calling `verify_password`, call `db.is_tried(&combination)`.
        - If `true`, skip.
        - After a failed `verify_password`, call `db.mark_as_tried(&combination)`.
- [x] Task: Implement success handling (printing and saving password) 9391edd
    - **Implementation Notes:**
        - If `verify_password` returns `true`, print to stdout with a clear message.
        - Use `std::fs::write` to save the password to `recovered_password.txt`.
        - Use `std::process::exit(0)`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Sequential Orchestration' (Protocol in workflow.md) 9391edd

## Phase 3: Parallelization & Optimization
Goal: Move to a multi-threaded architecture and optimize state persistence.

- [ ] Task: Implement `ParallelCracker` using a worker pool or `rayon` for concurrent verification
    - **Implementation Notes:**
        - Create `src/orchestration/parallel.rs`.
        - Use `rayon::iter::ParallelIterator` on the combinations.
        - Since `generate_combinations` returns an iterator, you might need to bridge it to a parallel iterator or collect chunks.
        - **Critical:** Ensure `SledDb` and `EncfSConfig` are shared safely (they should be `Arc` or have a `'static` lifetime).
- [ ] Task: Implement batched state updates in `ParallelCracker` to improve performance
    - **Implementation Notes:**
        - Instead of calling `db.mark_as_tried` for every failure, use a thread-local buffer or a channel to collect tried combinations.
        - Periodically (e.g., every 1000 attempts or every 1 second) batch-insert into `sled` using `db.db.transaction`.
- [ ] Task: Implement graceful shutdown and signal handling to ensure state consistency
    - **Implementation Notes:**
        - Use `ctrlc` crate.
        - On `SIGINT`, set a "stopping" flag that workers check.
        - Flush all pending batches to `sled` before exiting.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Parallelization & Optimization' (Protocol in workflow.md)

## Phase 4: CLI Integration & Final Polish
Goal: Connect the orchestration logic to the `clap` CLI and perform final testing.

- [ ] Task: Update `main.rs` to initialize and run the `ParallelCracker`
    - **Implementation Notes:**
        - Pass CLI arguments into `CrackerConfig`.
        - Initialize `SledDb`.
        - Call `ParallelCracker::run(config)`.
- [ ] Task: Conduct end-to-end integration tests with real EncfS config examples
    - **Implementation Notes:**
        - Create a test EncfS volume with a known password made of specific fragments.
        - Add an integration test in `tests/integration_cracker_test.rs` that runs the full tool and verifies the output.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: CLI Integration & Final Polish' (Protocol in workflow.md)
