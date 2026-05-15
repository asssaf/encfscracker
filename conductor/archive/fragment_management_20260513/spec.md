# Specification - Fragment Management & Groups

## Overview
This track implements a robust system for managing password fragments and organizing them into logical groups. The core innovation is the "Fragment Group" constraint: a generated password combination can contain at most one fragment from any specific group.

## Functional Requirements
- **Fragment Storage:** Persist fragments and their group assignments in the existing `sled` database.
- **Plain Text Support:** Handle standard UTF-8 text fragments.
- **Fragment Groups:**
    - Support creating named or indexed groups.
    - Enforcement: The combination generator must ensure that no more than one fragment from the same group is used in a single candidate password.
- **Input Methods:**
    - **CLI Flags:** Add fragments directly (e.g., `--add-fragment "word" --group "A"`).
    - **File Import:** Load fragments from a text file, optionally assigning them all to a specific group.
- **Management Commands:**
    - `list-fragments`: Display all fragments grouped by their ID.
    - `clear-fragments`: Remove all fragments or fragments from a specific group.

## Non-Functional Requirements
- **Efficiency:** The combination generator must efficiently skip invalid combinations based on group constraints to maintain high performance.
- **Persistence:** All fragment data must be saved atomically in `sled`.

## Acceptance Criteria
- [ ] Fragments can be added via CLI and file import.
- [ ] `list-fragments` correctly displays the fragment-to-group mapping.
- [ ] The cracking engine never generates a password containing two fragments from the same group.
- [ ] Data survives a tool restart.

## Out of Scope
- Automatic transformations (case toggling, leet-speak).
- Complex regex or pattern-based fragment generation.
