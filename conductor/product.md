# Initial Concept

a cli tool that cracks an encfs key (given the encfs config file) by using combinations of different password fragments. the tool maintains a list of alreay tried combinations so it can be stopped and resumed without repeating already tried ones.

# Product Definition - EncfS Key Cracker

## Vision
A high-performance CLI tool designed to recover EncfS keys by intelligently combining user-provided password fragments. It bridges the gap between manual guessing and exhaustive brute-force by leveraging partial knowledge, providing a persistent, secure, and resumable recovery process.

## Target Audience
- **Individual Users:** People who have lost access to their EncfS volumes but remember specific components of their password.
- **Security Professionals:** Forensic analysts and security auditors who need to recover data from encrypted EncfS volumes.
- **Advanced Users:** CLI-savvy users who require a flexible, automatable tool for password recovery.

## Core Goals
- **Intelligent Combination:** Generate and test password combinations based on user-provided fragments.
- **Resumability:** Maintain a persistent state of tried combinations to allow the process to be stopped and resumed without loss of progress.
- **Efficiency:** Optimize the cracking process for high performance, utilizing modern hardware capabilities.
- **Reliability:** Ensure compatibility with various EncfS configuration formats and encryption standards.
- **Security at Rest:** Protect sensitive user data, including password fragments and tried combinations, by encrypting them when stored on disk.

## Key Features
- **EncfS Compatibility:** Robust parsing and support for standard EncfS configuration files.
- **Fragment Management:** Flexible input system for defining password fragments (e.g., words, numbers, patterns).
- **Resume Support:** Integrated state management to track progress and prevent redundant testing.
- **High-Performance Engine:** Optimized cracking logic designed for speed and multi-core utilization.
- **Standard CLI Interface:** A professional command-line interface with clear flags, arguments, and progress reporting.
- **Encryption at Rest:** Secure storage of fragment lists and progress logs using industry-standard encryption.
