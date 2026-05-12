# Specification - Main Cracker Orchestration

## Overview
This track focuses on integrating the existing password fragment combination generator, the `sled`-based state management system, and the EncfS key verification logic into a cohesive, high-performance cracking engine. The goal is to produce a functional CLI tool that can reliably recover EncfS keys by testing combinations of user-provided fragments.

## Functional Requirements
- **Iterative Brute-force:** The cracker shall systematically test combinations of increasing lengths, starting from 1 fragment up to the total number of provided fragments.
- **Parallel Cracking:** The engine shall utilize multi-threading to parallelize the verification process across all available CPU cores.
- **Persistent Progress:**
    - The cracker shall use the `sled` database to track which combinations have already been tried.
    - Progress shall be recorded in batches to optimize disk I/O performance while maintaining a reasonable level of crash resilience.
    - Upon startup, the cracker shall resume from the last recorded state, skipping previously attempted combinations.
- **EncfS Verification:** The engine shall correctly derive keys and verify them against the provided `.encfs6.xml` configuration file.
- **Success Handling:** When the correct password is found, the cracker shall:
    - Display the password clearly on the standard output.
    - Save the recovered password to a file (e.g., `recovered_password.txt`).
    - Terminate immediately with a success exit code.
- **Interrupt Handling:** The cracker shall gracefully handle termination signals (e.g., Ctrl+C), ensuring that the current progress batch is committed to the database before exiting.

## Non-Functional Requirements
- **Performance:** The orchestration logic should minimize overhead, ensuring that the bottleneck remains the cryptographic operations (key derivation).
- **Reliability:** The state management must be robust against sudden crashes, ensuring no significant loss of progress.
- **Scalability:** The system should handle large numbers of fragments and deep combination depths efficiently.

## Acceptance Criteria
- [ ] Cracker can be started with an EncfS config file and a list of fragments.
- [ ] Cracker correctly identifies the valid password when it exists within the search space.
- [ ] Cracker can be stopped and resumed without repeating already tried combinations.
- [ ] Cracker utilizes multiple CPU cores during the cracking process.
- [ ] Cracker produces the correct output (file and console) upon success.

## Out of Scope
- Advanced fragment input (files, regex, etc.) - to be handled in a separate track.
- Detailed progress reporting/UI (ETA, throughput) - to be handled in a separate track.
- Support for non-EncfS encryption formats.
