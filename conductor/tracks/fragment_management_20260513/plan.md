# Implementation Plan - Fragment Management & Groups

This plan outlines the steps to implement a group-aware fragment management system using Sled.

## Phase 1: Data Model and Persistence [checkpoint: cf35071]
- [x] Task: Define the data structures for Fragments and Groups in `src/state/mod.rs`. 6dc6756
- [x] Task: Implement Sled storage logic for adding, retrieving, and clearing fragments/groups. 89825cf
- [x] Task: Write unit tests for fragment persistence. ba97a61
- [x] Task: Conductor - User Manual Verification 'Phase 1: Data Model and Persistence' (Protocol in workflow.md)

## Phase 2: CLI Management Commands [checkpoint: e6e9248]
- [x] Task: Update CLI arguments in `src/main.rs` to support `--add-fragment`, `--group`, and `--import-file`. 6633f95
- [x] Task: Implement the `list-fragments` command logic. bf44960
- [x] Task: Implement the `clear-fragments` command logic. bf44960
- [x] Task: Write integration tests for fragment CLI commands. 327119d
- [x] Task: Conductor - User Manual Verification 'Phase 2: CLI Management Commands' (Protocol in workflow.md)

## Phase 3: Group-Aware Combination Engine
- [ ] Task: Refactor `src/fragment_combination/mod.rs` to accept fragments with group IDs.
- [ ] Task: Implement the constraint logic: filter combinations to ensure max one fragment per group.
- [ ] Task: Update the parallel and sequential crackers to use the new constrained generator.
- [ ] Task: Write unit tests verifying that combinations respect group boundaries.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Group-Aware Combination Engine' (Protocol in workflow.md)

## Phase 4: Integration and Benchmarking
- [ ] Task: Perform an end-to-end test with a real EncfS config and grouped fragments.
- [ ] Task: Benchmark the performance impact of the group-filtering logic.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Integration and Benchmarking' (Protocol in workflow.md)
