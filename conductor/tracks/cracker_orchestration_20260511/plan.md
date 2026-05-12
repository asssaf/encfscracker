# Implementation Plan - Main Cracker Orchestration

## Phase 1: Foundation & EncfS Verification
Goal: Establish the core verification logic and prepare the integration points.

- [ ] Task: Implement EncfS configuration parsing and key derivation logic
- [ ] Task: Create a unified `CrackerConfig` struct to hold fragments, config, and state
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Foundation & EncfS Verification' (Protocol in workflow.md)

## Phase 2: Sequential Orchestration
Goal: Implement a single-threaded version of the cracker that integrates all components.

- [ ] Task: Implement `SequentialCracker` that iterates through combinations and verifies them
- [ ] Task: Integrate `SledDb` into `SequentialCracker` for persistence (individual checks)
- [ ] Task: Implement success handling (printing and saving password)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Sequential Orchestration' (Protocol in workflow.md)

## Phase 3: Parallelization & Optimization
Goal: Move to a multi-threaded architecture and optimize state persistence.

- [ ] Task: Implement `ParallelCracker` using a worker pool or `rayon` for concurrent verification
- [ ] Task: Implement batched state updates in `ParallelCracker` to improve performance
- [ ] Task: Implement graceful shutdown and signal handling to ensure state consistency
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Parallelization & Optimization' (Protocol in workflow.md)

## Phase 4: CLI Integration & Final Polish
Goal: Connect the orchestration logic to the `clap` CLI and perform final testing.

- [ ] Task: Update `main.rs` to initialize and run the `ParallelCracker`
- [ ] Task: Conduct end-to-end integration tests with real EncfS config examples
- [ ] Task: Conductor - User Manual Verification 'Phase 4: CLI Integration & Final Polish' (Protocol in workflow.md)
