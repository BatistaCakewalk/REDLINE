# 🗺️ REDLINE Roadmap

This document outlines the future direction of the REDLINE language and compiler.

## v0.9 - Memory, Overloads, & Polish

*   [x] **Automatic Memory Management**: Implemented `new` keyword and `std::shared_ptr` for all class objects to prevent memory leaks.
*   [x] **Function Overloading**: The compiler now supports defining multiple functions with the same name but different parameters.
*   [x] **Critical Safety Fixes**: Hardened the language by fixing out-of-bounds crashes and other memory-related bugs.
*   [x] **CLI Overhaul**: Rewrote the `redline.py` build script for a more professional and robust user experience.

## v1.0 - The Final Stretch

### Core Language Features
*   [x] **Dictionaries (`dict`)**: Add support for key-value pairs (Hash Maps) for fast lookups.
*   [x] **Loop Control**: Implement `break` and `continue` keywords for loops.
*   [x] **String Interpolation**: Support `f"Hello {name}"` syntax for easier string formatting.

### Standard Library Expansion
*   [x] **Command Line Arguments**: Allow programs to read arguments passed at startup (`sys.args`).
*   [x] **File System Operations**: Add `exists`, `remove`, and `list_dir` to the standard library.
*   [x] **Random Number Generation**: Add `random_int` and `random_float`.
*   [x] **Time Library**: Add functions to get the current time and measure duration.

### Tooling Ecosystem (we should add more things to this)
*   [ ] **Package Manager**: A simple tool to fetch and manage third-party REDLINE libraries.
*   [ ] **RedConfig Format**: A Project configuration tool to tell REDLINE where to compile, what RL Libraries are installed, and give special instructions.

### Critical Changes
*   [ ] **Redline.py to just Redline (idk how)**: Set up pathing for windows and linux/macs (so u can use "redline" everywhere)

## Long-Term Vision (Post v1.0)

*   [ ] **Cross-Platform Support**: Officially support and test on Windows (MSVC) and macOS (Clang).
*   [ ] **Language Server Protocol (LSP)**: Implement an LSP for better IDE integration (e.g., autocompletion, go-to-definition in VS Code, etc.).
*   [ ] **Concurrency**: Add support for multi-threading (`spawn`, `mutex`).
