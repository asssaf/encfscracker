# Implementation Plan - Build core EncfS key cracking engine

## Phase 1: Foundation & Parsing [checkpoint: 78e078e]
- [x] Task: Project Scaffolding 3e15c84
    - [x] Initialize Cargo project.
    - [x] Add dependencies (`clap`, `serde`, `quick-xml`, `anyhow`).
- [x] Task: EncfS Config Parsing 8fd066f
    - [x] Define Rust structs for `.encfs6.xml` schema.
    - [x] Implement parser and unit tests with sample config files.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Parsing' (Protocol in workflow.md)

## Phase 2: Combination Engine
- [x] Task: Fragment Combination Logic 384d477
    - [x] Implement an iterator/generator for permutations of password fragments.
    - [x] Unit tests for combination coverage.
- [x] Task: Parallel Execution Wrapper b3b3335
    - [x] Integrate `rayon` for parallelizing the combination testing loop.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Combination Engine' (Protocol in workflow.md) [checkpoint: 0114695]

## Phase 3: Cryptographic Validation [checkpoint: 19842ea]
- [x] Task: Key Derivation Function (KDF) Implementation 21a3287
    - [x] Implement PBKDF2/Scrypt (as specified in EncfS config) using `RustCrypto`.
- [x] Task: Master Key Validation 19842ea
    - [x] Implement the logic to decrypt and verify the master key using the derived key.
    - [x] Integration test with a small, known EncfS volume.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Cryptographic Validation' (Protocol in workflow.md)

## Phase 4: Basic CLI & State [checkpoint: 732888b]
- [x] Task: Command-Line Interface ed1fdee
    - [x] Implement `clap` arguments for config file and fragment input.
- [x] Task: Simple Progress Reporting 2a481af
    - [x] Implement a basic counter/speedometer for the CLI.
- [x] Task: Basic State Persistence 91195b6
    - [x] Implement saving/loading tried combinations to a simple file (placeholder for `sled`).

## Phase: Review Fixes
- [x] Task: Apply review suggestions b80ba65
