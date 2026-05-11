# Implementation Plan - Integrated State Management with Sled

## Phase 1: Foundation & Setup [checkpoint: f775e87]
- [x] Task: Integrate `sled` dependency and initialize database module (db1fdf9)
    - [ ] Add `sled` to `Cargo.toml`.
    - [ ] Create `src/state/sled_db.rs` and define the basic database structure.
    - [ ] Implement a singleton or thread-safe handle for the sled database.
- [x] Task: Define data schemas and partitioning strategy (1981515)
    - [ ] Define keys for tried combinations (e.g., hash or index-based).
    - [ ] Define keys for progress metrics (e.g., `current_batch_start`).
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Setup' (f775e87)

## Phase 2: Data Persistence [checkpoint: 0cf5546]
- [x] Task: Implement "Tried Combinations" storage (6517367)
    - [x] Write tests for inserting and checking combinations in Sled.
    - [x] Implement `mark_as_tried(combination)` in `sled_db.rs`.
    - [x] Implement `is_tried(combination)` for efficient lookups.
- [x] Task: Implement "Progress Metrics" storage (6475606)
    - [x] Write tests for saving and loading progress checkpoints.
    - [x] Implement `save_checkpoint(metrics)` and `load_checkpoint()`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Data Persistence' (0cf5546)

## Phase 3: Logic Integration
- [ ] Task: Integrate state lookups into the cracking engine
    - [ ] Modify the fragment combination loop to skip combinations where `is_tried` returns true.
    - [ ] Ensure parallel workers can efficiently check state without contention.
- [ ] Task: Implement resume-from-checkpoint logic
    - [ ] At startup, check for existing progress metrics.
    - [ ] Configure the combination generator to start from the saved checkpoint.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Logic Integration' (Protocol in workflow.md)

## Phase 4: Robustness & Verification
- [ ] Task: Implement robust recovery and corruption handling
    - [ ] Write tests simulating database corruption.
    - [ ] Implement error handling for sled operations.
    - [ ] Add a `--reset-state` flag to the CLI to clear the database.
- [ ] Task: Performance Benchmarking
    - [ ] Benchmark the cracking engine with and without state persistence.
    - [ ] Optimize lookups if performance degradation exceeds 10%.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Robustness & Verification' (Protocol in workflow.md)
