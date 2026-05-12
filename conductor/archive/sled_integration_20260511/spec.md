# Specification - Integrated State Management with Sled

## Overview
Implement a high-performance, persistent state management system using `sled` to track the progress of the EncfS key cracking process. This will replace the "minimal state persistence" of the core engine, allowing the tool to be stopped and resumed efficiently without repeating tried password combinations.

## Functional Requirements
- **Persistent Storage:** Use `sled` to store tried password combinations and high-level progress metrics.
- **Efficient Lookups:** Implement a partitioned database structure (e.g., using fragment indices or hashes) to ensure that checking if a combination has already been tried is extremely fast.
- **Resume Capability:** On startup, the tool must read the progress metrics from `sled` and resume the cracking process from the last known checkpoint.
- **Atomic Updates:** Ensure that updates to the database are atomic to prevent state corruption in case of unexpected termination.

## Non-Functional Requirements
- **Performance:** State lookups and updates should not significantly bottleneck the cracking engine's throughput.
- **Reliability:** Implement robust recovery mechanisms to handle potential database corruption or schema mismatches.
- **Portability:** The database should be stored in a consistent location relative to the EncfS volume or as specified by the user.

## Acceptance Criteria
- Successfully stop and resume a cracking session without repeating any previously tested password combinations.
- Demonstrate that the tool correctly identifies progress and starts from the correct fragment permutation.
- Verify that the state is preserved even after an abrupt process termination (e.g., SIGKILL).
- Maintain the performance of the cracking engine within 10% of the stateless implementation.

## Out of Scope
- Encryption of the state database (this will be addressed in a future "Encryption at Rest" track).
- Multi-machine synchronization of the state.
