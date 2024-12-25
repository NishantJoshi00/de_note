# DeNote

A powerful, terminal-based note-taking application with rich linking capabilities and an elegant TUI interface. Built in Rust for speed and reliability.

## Description

DeNote reimagines note-taking for power users and developers by providing:

- A beautiful terminal user interface that feels natural in your workflow
- A powerful linking system that helps you connect and organize your thoughts
- Local-first architecture ensuring privacy and quick access
- Decision-making tools to help structure your thinking process
- Blazing fast performance thanks to Rust

Whether you're documenting code, organizing research, or managing personal notes, DeNote provides a streamlined, keyboard-driven experience right in your terminal.

## Installation

### Prerequisites
- Rust toolchain and Cargo
- Terminal with Unicode support
- Git (for building from source)

### From Source
```bash
# Clone the repository
git clone https://github.com/NishantJoshi00/de_note.git
cd de_note

# Build from source
cargo build --release

# The binary will be available at target/release/de_note
```

### Via Cargo
```bash
cargo install de_note
```

## Usage

### Basic Commands
```bash
# Start DeNote
de_note

# Access help
de_note --help
```

For detailed usage instructions, please refer to our [User Guide](crates/de_note/docs/references.md).

## Features

- **Local-First Storage**: 
  - All notes stored locally on your machine
  - Fast access and complete privacy
  - No internet connection required

- **Rich Linking System**:
  - Create bidirectional links between notes
  - Forward and backward link navigation
  - Branch-based note organization
  - Decision trees and structured thinking tools

- **Terminal User Interface**:
  - Clean, responsive design using ratatui
  - Keyboard-driven navigation
  - Intuitive commands and shortcuts
  - Customizable themes and layouts

- **Fast and Efficient**:
  - Written in Rust for optimal performance
  - Handles large numbers of notes with ease
  - Quick search and filtering capabilities
  - Efficient memory usage

- **Note Organization**:
  - Hierarchical note structure
  - Tags and categories
  - Smart linking suggestions
  - Automatic backlinks

## Project Status

Current Version: 0.0.1-alpha

### Completed
- [x] Core note and link type definitions
- [x] Basic note creation and storage interface
- [x] Fundamental data structures

### In Progress
- [ ] Manager Interface Implementation
  - Storage Operations (Create, Read, Update, Delete)
  - Link Management
  - Branch Handling
- [ ] Storage Backend Development
- [ ] TUI Implementation

## Contributing Guidelines

1. **Issue First**: Create or find an issue before starting work
2. **Branch Naming**: Use descriptive branch names (feature/, bugfix/, etc.)
3. **Issue Tags**: 
   - [BUG] for bug reports
   - [FEATURE] for feature requests
   - [DOCS] for documentation improvements
4. **Code Style**: Follow Rust standard formatting guidelines
5. **Testing**: Ensure all tests pass before submitting PRs

To contribute:
1. Fork the repository
2. Create your feature branch
3. Implement your changes
4. Add tests where appropriate
5. Submit a pull request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- The Rust community for their incredible tools and support
- All contributors who help make DeNote better

---

Built with 🦀 in Rust
