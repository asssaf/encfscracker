# Implementation Plan - Build core EncfS key cracking engine

## Phase 1: Foundation & Parsing
- [x] Task: Project Scaffolding 3e15c84
    - [x] Initialize Cargo project.
    - [x] Add dependencies (`clap`, `serde`, `quick-xml`, `anyhow`).
- [ ] Task: EncfS Config Parsing
    - [ ] Define Rust structs for `.encfs6.xml` schema.
    - [ ] Implement parser and unit tests with sample config files.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Parsing' (Protocol in workflow.md)

## Phase 2: Combination Engine
- [ ] Task: Fragment Combination Logic
    - [ ] Implement an iterator/generator for permutations of password fragments.
    - [ ] Unit tests for combination coverage.
- [ ] Task: Parallel Execution Wrapper
    - [ ] Integrate `rayon` for parallelizing the combination testing loop.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Combination Engine' (Protocol in workflow.md)

## Phase 3: Cryptographic Validation
- [ ] Task: Key Derivation Function (KDF) Implementation
    - [ ] Implement PBKDF2/Scrypt (as specified in EncfS config) using `RustCrypto`.
- [ ] Task: Master Key Validation
    - [ ] Implement the logic to decrypt and verify the master key using the derived key.
    - [ ] Integration test with a small, known EncfS volume.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Cryptographic Validation' (Protocol in workflow.md)

## Phase 4: Basic CLI & State
- [ ] Task: Command-Line Interface
    - [ ] Implement `clap` arguments for config file and fragment input.
- [ ] Task: Simple Progress Reporting
    - [ ] Implement a basic counter/speedometer for the CLI.
- [ ] Task: Basic State Persistence
    - [ ] Implement saving/loading tried combinations to a simple file (placeholder for `sled`).
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Basic CLI & State' (Protocol in workflow.md)
