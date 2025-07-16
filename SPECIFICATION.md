# EBML File Format Specification for Game Case Containers

**Version:** 1.0  
**Date:** 2025-15-25  
**Author:** Tyler Bibeault  
**Based On:** EBML (Extensible Binary Meta Language)

---

### 1. Introduction

#### 1.1. Purpose
This document describes the structure of the `.gcase` file format, which is based on the Extensible Binary Meta Language (EBML). The purpose of this specification is to provide a definitive guide for developers who need to create, parse, or write `.gcase` files.

#### 1.2. Scope
This specification covers version `1.0` of the `Game Case` container format. It defines the mandatory EBML Header and the hierarchical structure of all custom elements used in the file.

#### 1.3. Target Audience
This document is intended for software developers and engineers. A basic understanding of binary data structures is assumed. Familiarity with the EBML specification is recommended but not required.

#### 1.4. Document Conventions

* **Elements:** The file is composed of EBML Elements. Each element consists of an **Element ID**, a **Size**, and **Data**.
* **VINT (Variable-Size Integer):** Both the Element ID and the Size are encoded as variable-size integers (VINTs) to save space. See Section 4.1 for details.
* **Element ID:** A unique VINT that identifies an element's type and purpose.
* **Master Element:** An element that contains other elements as its data, forming a hierarchical tree.
* **Endianness:** All integer and float data types are stored in **Big Endian** format.

---

### 2. General File Structure

A `.gcase` file is an EBML document. It begins with a mandatory `EBML` Header element, followed by a single Master element, which for this file container is defined as `GameCase`.

**Logical Hierarchy:**

```
EBML File
|
+-- EBML Header (Mandatory)
|   |
|   +-- EBMLVersion
|   +-- DocType
|   +-- DocTypeVersion
|   +-- ...
|
+-- GameCase (Master Element)
    |
    +-- Game (Master Element)
    |   |
    |   +-- Game Title
    |   +-- Release Date
    |   +-- Developer
    |   +-- Publisher
    |   +-- Genre
    |   +-- Description
    |   +-- ...
    |
    |
    +-- Manuals (Master Element)
    |   |
    |   +-- Manual 1 (Master Element)
    |   |   |
    |   |   +-- Region
    |   |   +-- Language
    |   |   +-- Page Count
    |   |   +-- Variant
    |   |   +-- Revision Number
    |   |   +-- Format
    |   |   +-- ...
    |   |
    |   |
    |   |
    |   +-- Manual n (Master Element)
    |
    |
    +-- Box Art (Master Element)
        |
        +-- IndexEntry 1
        +-- IndexEntry 2
        +-- ...
```

**Physical Layout of an Element:**

```
+--------------------+----------------+---------------------------+
|  Element ID (VINT) |  Size (VINT)   |  Data (Size bytes long)   |
+--------------------+----------------+---------------------------+
```

---

### 3. EBML Element Definitions

This section defines the specific elements used in the `[Your DocType Name]` DocType.

#### 3.1. EBML Header

This is the standard EBML Header and **must** be the first element in the file.

| Element Name     | Element ID | Type            | Cardinality | Description                                    |
| :--------------- | :--------- | :-------------- | :---------- | :--------------------------------------------- |
| `EBML`           | `0x1A45DFA3` | Master Element  | 1           | The container for all EBML header elements.    |
| &nbsp;&nbsp;`EBMLVersion`    | `0x4286`     | Unsigned Int    | 1           | The version of the EBML parser required (e.g., 1). |
| &nbsp;&nbsp;`DocType`        | `0x4282`     | String          | 1           | The DocType of this file: `"[Your DocType Name]"`. |
| &nbsp;&nbsp;`DocTypeVersion` | `0x4287`     | Unsigned Int    | 1           | The version of the DocType that was written (e.g., 1). |
| &nbsp;&nbsp;`DocTypeReadVersion`|`0x4285`  | Unsigned Int    | 1           | The minimum DocType version a parser must support. |

#### 3.2. Root Element: `GameCase`

This element acts as the main container for the rest of the file's data.

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `GameCase`           | `[e.g., 0x18538067]`| Master Element| 1   | The top-level element containing all file data.|

##### **Children of `GameCase`:**

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Game`               | `[e.g., 0x1549A966]`| Master Element| 1   | Contains metadata about the file content.     |
| `Manuals`            | `[e.g., 0x1F43B675]`| Master Element| 0..1| Contains one or more blocks of primary data.  |
| `RomHacks`           | `[e.g., 0x1C53BB6B]`| Master Element| 0..1| Contains pointers for fast seeking.           |
| `FanArt`             | `0xn`      | Master Element | 0..1        |  Contains fan art for the subject game of this file.|
| `Videos`             | `0xn`      | Master Element | 0..1        | Contains videos of the subject game of this file.|

#### 3.3 `Game` Element

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Title`              | `[e.g., 0x1549A966]`| UTF-8 String| 1   | Container for file metadata.                  |
| `Developer`          | `[e.g., 0x4489]`| UTF-8 String| 1..n      | The developer of the game content.            |
| `Genre`              | `0xn`      | UTF-8 String   | 0..1        | The genre of the game content.                |
| `Minimum Player Count`| `0xn`     | Unsigned Int   | 0..1        | The minimum number of players required to play the game.|
| `Maximum Player Count`| `0xn`     | Unsigned Int   | 0..1        | The maximum number of players allowed to play the game.|
| `Online Player Rating`| `0xn`     | Unsigned Int   | 0..1        | The user or critical rating of the game.      |
| `Game System`        | `0xn`      | UTF-8 String   | 1           | The game system for the subject game and it's data.|
| `Region Info`        | `0xn`      | Master Element | 0..n        | Container for the region specific information about the game.|
| `GameData`           | `0xn`      | Master Element | 0..n        | Container for storing game binary data.       |

##### 3.3.1 **Children of `RegionInfo`:**

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Region`             | `0xn`      | UTF-8 String   | 1           | The region of this `Region` element.           |
| `Language`           | `0xn`      | UTF-8 String   | 1..n        | The language of this `Region` element.          |
| `Release Date`       | `0xn`      | Date           | 1           | The release date of this `Region` element.      |
| `Publisher`          | `0xn`      | UTF-8 String   | 1           | The publisher of the game content of this `Region` element.|
| `Age Rating`         | `0xn`      | UTF-8 String   | 1           | The age rating of the game content of this `Region` element.|

##### 3.3.2 `GameData` Element
This element is the container for all game binary data. It can hold multiple `GameDataEntry` elements, which allows for storing different versions or types of game data (e.g., a raw ROM and a compressed CHD version) within the same `.gcase` file.

| Element Name | Element ID          | Type           | Cardinality | Description                                  |
| :----------- | :------------------ | :------------- | :---------- | :------------------------------------------- |
| `GameData`   | `[e.g., 0x16549A66]` | Master Element | 0..1        | Container for one or more game data entries. |

###### **Children of `GameData`:**
| Element Name      | Element ID          | Type           | Cardinality | Description                                   |
| :---------------- | :------------------ | :------------- | :---------- | :-------------------------------------------- |
| `GameDataEntry`   | `[e.g., 0x1749A966]`| Master Element | 1..n        | A container for a single instance of game data (e.g., a single ROM, a CHD set, or an archive). |

#### 3.3.1.2 `GameDataEntry` Element
This is a self-contained entry for a game's binary data. The `DataFormat` element within it **must** be parsed first, as it defines which other elements are present within this entry.

| Element Name  | Element ID          | Type           | Cardinality | Description                                   |
| :------------ | :------------------ | :------------- | :---------- | :-------------------------------------------- |
| `DataFormat`  | `[e.g., 0x5001]`    | UTF-8 String   | 1           | Declares the format of the data. Must be one of: "RAW", "ARCHIVE", "CHD", "BIN/CUE". |
| `RawData`     | `[e.g., 0x5100]`    | Master Element | 0..1        | Contains data for a single raw game file. **Present if `DataFormat` is "RAW".** |
| `ArchiveData` | `[e.g., 0x5200]`    | Master Element | 0..1        | Contains data for a compressed archive. **Present if `DataFormat` is "ARCHIVE".** |
| `ChdData`     | `[e.g., 0x5300]`    | Master Element | 0..1        | Contains data for a CHD collection. **Present if `DataFormat` is "CHD".** |
| `BinCueData`  | `[e.g., 0x5400]`    | Master Element | 0..1        | Contains data for a BIN/CUE collection. **Present if `DataFormat` is "BIN/CUE".** |
| `SSMCData`    | `[e.g., 0x5500]`    | Master Element | 0..1        | Contains data for a SpriteShrink MultiCart (.ssmc). **Present if `DataFormat` is "SSMC".** 

##### 3.3.4.1 `RawData` Element (Format: "RAW")
Stores a single, uncompressed game file.

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ROMFileSize`        | `0xn`      | Unsigned Int   | 1           | The size of the ROM file in bytes.            |
| `ROMFileRegion`      | `0xn`      | UTF-8 String   | 1           | The region of this game file.                 |
| `SupportedLanguage`  | `0xn`      | UTF-8 String   | 0..n        | A supported language by the ROM.              |
| `CRC32`              | `0xn`      | Binary         | 0..1        | The 4-byte CRC32 hash of the file.            |
| `MD5`                | `0xn`      | Binary         | 0..1        | The 16-byte MD5 hash of the file.             |
| `SHA1`               | `0xn`      | Binary         | 0..1        | The 20-byte SHA-1 hash of the file.           |
| `SHA-256 Hash`       | `0xn`      | Unsigned Int   | 0..1        | The SHA-256 hash of the file.                 |
| `SHA-512 Hash`       | `0xn`      | Unsigned Int   | 0..1        | The SHA-512 hash of the file.                 |
| `ROMData`            | `0xn`      | Binary         | 1           | The raw binary data of the game ROM.          |

##### 3.3.4.2 `ArchiveData` Element (Format: "ARCHIVE")
Stores a single compressed file (zip, 7z, etc.) that may contain one or multiple ROMs. This allows inspecting archive contents without full decompression.

| Element Name            | Element ID | Type           | Cardinality | Description                               |
| :---------------------- | :--------- | :------------- | :---------- | :---------------------------------------- |
| `CompressionAlgorithm`  | `0xn`      | UTF-8 String   | 1           | The compression algorithm (e.g., "zip", "7z"). |
| `ArchivedFileCount`     | `0xn`      | Unsigned Int   | 1           | The number of files described in the metadata. |
| `ArchivedFileMetadata`  | `0xn`      | Master Element | 0..n        | Metadata for a single file within the archive. |
| `ArchiveBinary`         | `0xn`      | Binary         | 1           | The complete binary data of the archive file. |

**Children of `ArchivedFileMetadata`:**
| Element Name      | Element ID | Type           | Cardinality | Description                           |
| :---------------- | :--------- | :------------- | :---------- | :------------------------------------ |
| `ROMFileName`     | `0xn`      | UTF-8 String   | 1           | The full filename within the archive. |
| `ROMFileSize`     | `0xn`      | Unsigned Int   | 1           | The uncompressed size of the file.    |
| `ROMFileRegion`   | `0xn`      | UTF-8 String   | 1           | The region of this game file.         |
| `SupportedLanguage`| `0xn`     | UTF-8 String   | 1..n        | A supported language by the ROM.      |
| `CRC32`           | `0xn`      | Binary         | 0..1        | The 4-byte CRC32 hash of the file.    |
| `MD5`             | `0xn`      | Binary         | 0..1        | The 16-byte MD5 hash of the file.     |
| `SHA1`            | `0xn`      | Binary         | 0..1        | The 20-byte SHA-1 hash of the file.   |
| `SHA-256 Hash`    | `0xn`      | Unsigned Int   | 0..1        | The SHA-256 hash of the file.         |
| `SHA-512 Hash`    | `0xn`      | Unsigned Int   | 0..1        | The SHA-512 hash of the file.         |


##### 3.3.4.3 `ChdData` Element (Format: "CHD")
Stores a collection of CHD files, typically for multi-disc games.

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `CollectionRegion`   | `0xn`      | UTF-8 String   | 1        | The primary region of this collection.           |
| `SupportedLanguage`  | `0xn`      | UTF-8 String   | 1..n        | A language supported by this set.             |
| `CHDCount`           | `0xn`      | Unsigned Int   | 1           | The number of CHD files in this collection.   |
| `CHDEntry`           | `0xn`      | Master Element | 1..n        | A container for a single CHD file.            |

**Children of `CHDEntry`:**
| Element Name | Element ID | Type           | Cardinality | Description                                   |
| :----------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `DiscNum`    | `0xn`      | Unsigned Int   | 1           | The disc number for this CHD in the collection. |
| `CRC32`      | `0xn`      | Binary         | 0..1        | The 4-byte CRC32 hash of the uncompressed data. |
| `MD5`        | `0xn`      | Binary         | 0..1        | The 16-byte MD5 hash of the uncompressed data.  |
| `SHA1`       | `0xn`      | Binary         | 0..1        | The 20-byte SHA-1 hash of the uncompressed data.|
| `SHA-256 Hash`| `0xn`     | Unsigned Int   | 0..1        | The SHA-256 hash of the uncompressed data.    |
| `SHA-512 Hash`| `0xn`     | Unsigned Int   | 0..1        | The SHA-512 hash of the uncompressed data.    |
| `CHDBinary`  | `0xn`      | Binary         | 1           | The binary data of the complete .chd file.    |


##### 3.3.4.4 `BinCueData` Element (Format: "BIN/CUE")
Stores a collection of BIN/CUE file pairs, typically for multi-disc games.

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `CollectionRegion`   | `0xn`      | UTF-8 String   | 1           | The primary region of this collection.        |
| `SupportedLanguage`  | `0xn`      | UTF-8 String   | 1..n        | A language supported by this set.             |
| `BinCueCount`        | `0xn`      | Unsigned Int   | 1           | The number of BIN/CUE pairs in this collection. |
| `BinCueEntry`        | `0xn`      | Master Element | 1..n        | A container for a single BIN/CUE pair.        |

**Children of `BinCueEntry`:**
| Element Name  | Element ID | Type           | Cardinality | Description                                   |
| :------------ | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `DiscNum`     | `0xn`      | Unsigned Int   | 1           | The disc number for this BIN file.            |
| `CueSheet`    | `0xn`      | UTF-8 String   | 1           | The full text content of the .cue sheet.      |
| `CRC32`       | `0xn`      | Binary         | 0..1        | The 4-byte CRC32 hash of the .bin file.       |
| `MD5`         | `0xn`      | Binary         | 0..1        | The 16-byte MD5 hash of the .bin file.        |
| `SHA1`        | `0xn`      | Binary         | 0..1        | The 20-byte SHA-1 hash of the .bin file.      |
| `SHA-256 Hash`| `0xn`      | Unsigned Int   | 0..1        | The SHA-256 hash of the file.                 |
| `SHA-512 Hash`| `0xn`      | Unsigned Int   | 0..1        | The SHA-512 hash of the file.                 |
| `BinBinary`   | `0xn`      | Binary         | 1           | The binary data of the complete .bin file.    |

##### 3.3.4.5 `SSMCData` Element (Format: "SSMC")
Stores the metadata of an SpriteShrink MultiCart archive, typically contains all regsions and revisions of a single game.

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ArchivedFileCount`  | `0xn`      | Unsigned Int   | 1           | The number of files described in the metadata.|
| `SSMCEntry`          | `0xn`      | Master Element | 1..n        | Metadata for a file in the .ssmc file.        |
| `SSMCBinary`         | `0xn`      | Binary         | 1           | The complete binary data of the .ssmc file.   |

**Children of `BinCueEntry`:**
| Element Name  | Element ID | Type           | Cardinality | Description                                   |
| :------------ | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ROMFileName` | `0xn`      | UTF-8 String   | 1           | The full filename within the archive.         |
| `ROMFileSize` | `0xn`      | Unsigned Int   | 1           | The uncompressed size of the file.            |
| `ROMFileRegion`| `0xn`     | UTF-8 String   | 1           | The region of this game file.                 |
| `SupportedLanguage`| `0xn` | UTF-8 String   | 1..n        | A language supported by this set.             |
| `ROMFileIndex`| `0xn`      | Unsigned Int   | 1           | The index of the file in the .ssmc data.      |
| `CRC32`       | `0xn`      | Binary         | 0..1        | The 4-byte CRC32 hash of the compressed file. |
| `MD5`         | `0xn`      | Binary         | 0..1        | The 16-byte MD5 hash of the compressed file.  |
| `SHA1`        | `0xn`      | Binary         | 0..1        | The 20-byte SHA-1 hash of the file.           |
| `SHA-256 Hash`| `0xn`      | Unsigned Int   | 0..1        | The SHA-256 hash of the file.                 |
| `SHA-512 Hash`| `0xn`      | Unsigned Int   | 0..1        | The SHA-512 hash of the file.                 |

#### 3.4 `Manuals` Element

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ManualEntry`        | `0xn`      | Master Element | 1..n        | Contains data for a single manual.            |

**Children of `ManualEntry`:**
| Element Name  | Element ID | Type           | Cardinality | Description                                   |
| :------------ | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Region`      | `0xn`      | UTF-8 String   | 1           | Region where the manual was printed.          |
| `Language`    | `0xn`      | UTF-8 String   | 1..n        | A language supported by the manual.           |
| `Page Count`  | `0xn`      | Unsigned Int   | 1           | The amount of pages the manual contains.      |
| `Revision`    | `0xn`      | UTF-8 String   | 1           | The revision number(s)/letter(s) of the manual.|
| `PDFData`     | `0xn`      | Binary         | 0..1        | The PDF binary data of the manual.            |
| `CBZData`     | `0xn`      | Binary         | 0..1        | The CBZ binary data of the manual.            |
| `CBRData`     | `0xn`      | Binary         | 0..1        | The CBR binary data of the manual.            |

#### 3.5 `RomHacks` Element

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| 

#### 3.6 `Media` Element

| Element Name         | Element ID | Type           | Cardinality | Description                                   |
| :------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `FanArt`             | `0xn`      | Master Element | 0..1        |  Contains fan art for the subject game of this file.|


---

### 4. Data Types

#### 4.1. VINT (Variable-Size Integer)

A VINT is a method of encoding an integer into a variable number of bytes. The number of leading zeros in the first byte determines the total length of the integer (from 1 to 8 bytes). This allows smaller values to use less space.

| First Byte (Binary) | Length (bytes) | Value Range                    |
| :------------------ | :------------- | :----------------------------- |
| `1xxxxxxx`          | 1              | 0 to 126                       |
| `01xxxxxx xxxxxxxx` | 2              | 127 to 16,383                  |
| `001xxxxx ...`      | 3              | 16,384 to 2,097,151            |
| `...`               | ...            | ... up to 8 bytes              |

#### 4.2. EBML Data Types

| Type Name      | Description                                                 |
| :------------- | :---------------------------------------------------------- |
| **Signed Int** | A big-endian, signed integer. Can be 1-8 bytes.             |
| **Unsigned Int** | A big-endian, unsigned integer. Can be 1-8 bytes.           |
| **Float** | A big-endian, IEEE 754 floating point number. 4 or 8 bytes. |
| **String** | A printable ASCII string (0x20 to 0x7E). Not null-terminated. |
| **UTF-8 String** | A Unicode string encoded with UTF-8. Not null-terminated.   |
| **Date** | A 64-bit signed integer representing nanoseconds since Jan 1, 2001, 00:00:00 UTC. |
| **Binary** | Raw binary data that is not interpreted by the parser.      |
| **Master Element** | Contains other EBML Elements. The element's data is a sequence of sub-elements. |

---

### 5. Version History

| Version | Date       | Author(s) | Summary of Changes                               |
| :------ | :--------- | :-------- | :----------------------------------------------- |
| 1.0     | 2023-10-27 | [Your Name] | Initial version of the specification.            |

