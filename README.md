# DeNote: A Terminal-based Note-Taking Application

DeNote is a powerful, fast, and intuitive note-taking application that runs in your terminal. Built with Rust, it offers a beautiful text-based user interface (TUI) powered by [ratatui](https://github.com/tui-rs-revival/ratatui), allowing for a seamless note-taking experience right from your command line.

## Features

- **Local Storage**: All your notes are stored locally, ensuring privacy and quick access.
- **Forward Links**: Easily create and navigate through links between your notes.
- **Decision Making**: Integrated tools to help with decision-making processes.
- **Beautiful TUI**: A clean and responsive terminal user interface for a great note-taking experience.
- **Fast and Efficient**: Leveraging Rust's performance to handle large numbers of notes with ease.

## Progress

> Current Version: 0.0.1-alpha

### Completed Milestones

- [x] Creating types for notes and links
- [x] Basic note creation and storage interface definitions

### In Progress

- [ ] Implementation of manager interfaces
  - [ ] Add Interface
    - [x] Add Note
    - [x] Add Link
    - [ ] Add Branch
  - [ ] Update Interface
    - [ ] Update Note
    - [ ] Update Link
    - [ ] Update Branch
  - [ ] Delete Interface
    - [ ] Delete Note
    - [ ] Delete Link
    - [ ] Delete Branch
  - [ ] Read Interface
    - [ ] Read Note
    - [ ] Read Link
    - [ ] Read Branch
- [ ] Implementation of note storage
  - [ ] File-based storage
  - [ ] Database-based storage
- [ ] Implementation of notes tui

## Installation

Ensure you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

Then, you can install DeNote using cargo:

```bash
cargo install de_note
```

Or clone the repository and build from source:

```bash
git clone https://github.com/NishantJoshi00/de_note.git
cd de_note
cargo build --release
```

## Usage

To start DeNote, simply run:

```bash
de_note
```

To understand how to use DeNote, you can refer to the [User Guide](crates/de_note/docs/references.md).

## Contributing

We welcome contributions to DeNote! Please feel free to submit issues, fork the repository and send pull requests!

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) for the amazing TUI library
- All contributors who have helped shape DeNote

---

Built with ❤️ using Rust
