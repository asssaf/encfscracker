# Product Guidelines - EncfS Key Cracker

## Design Philosophy
The EncfS Key Cracker is a focused, high-performance utility designed for reliability and security. It prioritizes efficient execution and clear status reporting while ensuring that sensitive recovery data is protected.

## User Experience (UX)
- **High Visibility:** The tool must provide real-time feedback on its progress, including the current cracking speed (combinations/sec) and the estimated time remaining or total combinations tried.
- **Clear Errors:** Provide precise and actionable error messages for common issues such as malformed EncfS configuration files, invalid fragment inputs, or file permission errors.
- **Safe Interruptibility:** The tool must handle interrupts (e.g., Ctrl+C) gracefully, ensuring that the current state is saved before exiting so progress is not lost.
- **Terse Output:** By default, the CLI output should be concise and focused on essential information, avoiding unnecessary chatter.

## CLI Standards
- **Naming Convention:** Use `kebab-case` for all long-form command-line arguments and flags (e.g., `--config-path`, `--fragment-file`).
- **Short Flags:** Provide standard single-letter short flags for commonly used arguments (e.g., `-c` for `--config-path`).
- **Standard CLI Interaction:** Follow standard Unix/Linux CLI conventions for flags, arguments, and return codes.

## Logging and Debugging
- **Default Minimalist Logging:** By default, logs should only capture critical errors and the final recovery result to keep output clean.
- **Optional Verbosity:** Provide a `--verbose` or `-v` flag to enable detailed logging, including information about combination generation, EncfS configuration parsing, and internal state management.

## Security Guidelines
- **Encryption at Rest:** All sensitive local storage, including fragment lists and progress logs, must be encrypted using industry-standard algorithms.
- **Minimal Exposure:** Ensure that password combinations are not leaked through logs or process monitoring tools during execution.
