# GameCase File Format

## Overview

**GameCase (`.gcase`)** is a specialized, open-source file format designed to be an all-in-one container for video game data. Based on the robust and extensible EBML (Extensible Binary Meta Language) standard—the same technology that powers the Matroska video format (`.mkv`)—GameCase provides a structured and future-proof way to archive, store everything related to a single game.

Whether you are a developer, a preservationist, or an enthusiast, the GameCase format allows you to bundle game ROMs, metadata, manuals, box art, and even romhacks into a single, organized file.

## Features

The GameCase format is designed to be comprehensive, supporting a wide array of data types relevant to video game archiving:

* **Core Game Data:**
    * Store game binaries in multiple formats, including raw ROMs, compressed archives (`.zip`, `.7z`), CHD (Compressed Hunks of Data), and BIN/CUE pairs.
    * Support for multiple game versions (e.g., regional releases, revisions) within a single file.
    * Support SSMC archives made by the SpriteShrink tool (found here: [https://github.com/Zade222/SpriteShrink](https://github.com/Zade222/SpriteShrink))
* **Rich Metadata:**
    * Detailed game information such as title, developer, publisher, release dates, and genre.
    * Region-specific details, including languages, publishers, and age ratings.
    * Player count, game system, and user/critical ratings.
* **Game Manuals:**
    * Embed digital copies of game manuals in various formats (e.g., PDF, CBZ).
    * Include metadata for each manual, such as region, language, and revision number.
* **Optimized for Fast Access:** 
    * Includes an index for fast seeking of content within the file. This is especially useful for accessing files on slow media or over a network transfer.
* **Media Collections:**
    * Store related media like box art (front, back), cartridge/disc images, fan art, and screenshots.
    * Embed video content such as trailers and gameplay clips.
* **Romhacks and Patches:**
    * Archive patch files (e.g., IPS, BPS) along with metadata like hack title, author, and target ROM hashes.
* **Extensible by Design:**
    * Built on EBML, the format can be extended with new elements in the future without breaking compatibility with older files.

## Specification

This project is defined by a formal specification that details the hierarchical structure, element IDs, and data types. For developers who wish to implement a parser or writer for the `.gcase` format, the specification is the definitive guide.

* [**Read the full GameCase File Format Specification**](./SPECIFICATION.md)

## Usage

As a file format specification, there is no software to "install." Instead, developers can use the provided specification to create their own tools, such as:

* **Game library managers** that can read and display information from `.gcase` files.
* **Converters** to package existing ROMs and metadata into the `.gcase` format.
* **Emulators** that can directly load games from a `.gcase` container.

## Author

* **Tyler Bibeault**

## License

The GameCase file format specification is open and free to use. It is licensed under the **Creative Commons Attribution 4.0 International License**.

You are free to:
* **Share** — copy and redistribute the material in any medium or format.
* **Adapt** — remix, transform, and build upon the material for any purpose, even commercially.

Under the following terms:
* **Attribution** — You must give appropriate credit, provide a link to the license, and indicate if changes were made.

For the full license text, please see the [LICENSE.md](./LICENSE.md) file.