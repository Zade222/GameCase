# GameCase: A Modern Game Archiving Toolkit

![Rust](https://github.com/rust-lang/rust/actions/workflows/rust.yml/badge.svg)

**GameCase is a comprehensive project for creating, managing, and parsing `.gcase` game archives. It provides both a detailed file format specification and a powerful interactive terminal application to bring your game assets together.**

This repository contains two primary components:

1.  **`game_case_creator`**: An interactive terminal application (TUI) for creating and managing `.gcase` files. It's built with [Cursive](https://github.com/gyscos/Cursive), allowing it to run on your desktop or remotely over SSH.
2.  **The `.gcase` Specification**: A formal specification for a flexible, EBML-based file format designed to archive game assets efficiently.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Usage](#usage)
  - [Remote Usage (via SSH)](#remote-usage-via-ssh)
- [For Developers](#for-developers)
  - [Using the Parser Library](#using-the-parser-library)
- [The `.gcase` Specification](#the-gcase-specification)
- [Contributing](#contributing)
- [License](#license)

## Overview

The goal of the GameCase project is to provide a robust and user-friendly solution for packaging game data. The interactive terminal interface makes it easy to create archives, whether you're working on your local machine or managing a game library on a remote server.

The `.gcase` format is built upon **EBML (Extensible Binary Meta Language)**, the same container format used by Matroska (`.mkv`). This provides a future-proof foundation that is both flexible and efficient.

## Features

-   **Interactive Terminal UI**: A user-friendly, Cursive-based interface that works locally or over an SSH connection, perfect for managing archives on a remote server or NAS.
-   **Modern File Format**: A detailed specification for the `.gcase` archive format.
-   **Developer-Friendly Library**: A dedicated parser library (`lib_game_case_parser`) allows for easy integration into your own Rust projects.
-   **Cross-Platform**: Build and use on Windows, macOS, and Linux.

## SpriteShrink Archives

SpriteShrink is a key feature of GameCase, designed to intelligently manage game collections that include multiple regional or revised versions of the same game (e.g., USA, Europe, Japan, Rev 1).

Instead of storing multiple, nearly-identical ROMs, a SpriteShrink archive deduplicates the common data and stores only the differences. This allows you to:

-   **Save Space**: Significantly reduce the storage footprint of your game library.
-   **Simplify Management**: Keep all variants of a single game within one managed archive.
-   **Streamline ROM Hacking**: Easily combine a specific game version with a ROM hack, as some hacks require a highly specific revision of a game to function correctly.

For more information on the SpriteShrink archive and tool check out the source here: [https://github.com/Zade222/SpriteShrink](https://github.com/Zade222/SpriteShrink)

## Project Structure

This project is a Rust workspace containing multiple crates, each with its own license:

-   `crates/game_case_creator`: The main TUI application. **(Licensed under GPLv3)**
-   `crates/lib_game_case_parser`: The core library for parsing `.gcase` files. **(Licensed under MPL-2.0)**
-   `SPECIFICATION.md`: The official document detailing the `.gcase` file format. **(Licensed under CC BY 4.0)**

## Building from Source

To build and run `game_case_creator`, you will need to have the Rust toolchain installed. You can install it from [rustup.rs](https://rustup.rs/).

1.  Clone the repository:
    ```sh
    git clone https://github.com/Zade222/GameCase.git
    cd gamecase
    ```

2.  Build the application in release mode:
    ```sh
    cargo build --release --package game_case_creator
    ```

The compiled binary will be located at `target/release/game_case_creator`.

## Usage

Simply run the application from your terminal:

```sh
game_case_creator
```

This will launch the interactive terminal user interface (TUI). From here, you can:
- Navigate your file system.
- Select a source directory containing your game assets.
- Specify an output path and filename for your `.gcase` archive.
- Build the archive directly from the interface.
    <!--- Put a screenshot example of the interface. -->

### Remote Usage (via SSH)

One of the key advantages of the TUI is that you can run it on a remote machine (like a home server or NAS) and interact with it seamlessly over an SSH connection, allowing you to manage your game archives from anywhere.

## For Developers

## The `.gcase` Specification

For a deep dive into the file format's structure, element IDs, and low-level details, please read the official **[GameCase Specification](SPECIFICATION.md)**.

## Contributing

Contributions are welcome! If you'd like to help improve GameCase, please feel free to fork the repository, make changes, and submit a pull request. If you find a bug or have a feature request, please open an issue.

Please understand that the issue tracker isn't a forum. It is for tracking feature requests, problems, issues, etc.

If you want to discuss the software and it's capabilities with others please use the [GitHub Discussions page](https://github.com/Zade222/GameCase/discussions).

## License

This project utilizes multiple licenses for its different components. Please ensure you understand and comply with the license for any part of the project you use.

-   **TUI Application (`game_case_creator`)** is licensed under the **[GNU General Public License v3.0 (GPLv3)](https://www.gnu.org/licenses/gpl-3.0.en.html)**.
    -   This is a copyleft license, which means that any derivative works of the application must also be licensed under the GPLv3.

-   **Parser Library (`lib_game_case_parser`)** is licensed under the **[Mozilla Public License 2.0 (MPL-2.0)](https://www.mozilla.org/en-US/MPL/2.0/)**.
    -   This is a "weak copyleft" license. It requires that modifications to the library's source files be shared under the MPL-2.0, but it allows you to link the library into a larger work under a different license.

-   **Specification (`SPECIFICATION.md`)** is licensed under the **[Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/)**.
    -   This license allows you to share and adapt the specification for any purpose, even commercially, as long as you give appropriate credit.
