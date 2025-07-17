# EBML File Format Specification for Game Case Containers

**Version:** -  
**Date:** 2025-07-16  
**Author:** Tyler Bibeault  
**Based On:** EBML (Extensible Binary Meta Language)

---

### 1. Introduction

#### 1.1. Purpose
This document describes the structure of the `.gcase` file format, which is based on the Extensible Binary Meta Language (EBML). The purpose of this specification is to provide a definitive guide for developers who need to create, parse, or write `.gcase` files.

#### 1.2. Scope
This specification covers version `-` of the `Game Case` container format. It defines the mandatory EBML Header and the hierarchical structure of all custom elements used in the file.

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
    |
    +-- GameCaseVersion (Mandatory)
    |
    |
    +-- Index (Master Element)
    |   |
    |   +-- Index Entry 1 (Master Element)
    |   |   |
    |   |   +-- TargetID 
    |   |   +-- Index Position
    |   |   +-- ...
    |   |
    |   +-- Index Entry n (Master Element)
    |
    |
    +-- Game (Master Element)
    |   |
    |   +-- Title
    |   +-- Developer
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
    |   +-- Manual n (Master Element)
    |
    |
    +-- Romhacks (Master Element)
    |   |
    |   +-- Romhack 1 (Master Element)
    |   |   |
    |   |   +-- HackTitle
    |   |   +-- HackAuthor
    |   |   +-- ...
    |   |
    |   +-- Romhack n (Master Element)
    |
    |
    +-- Media (Master Element)
        |
        +-- ImageCollection 1
        +-- VideoCollection 1
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

This section defines the specific elements used in the `GameCase` DocType.

#### 3.1. EBML Header

This is the standard EBML Header and **must** be the first element in the file. The element IDs are fixed and can not be changed per the EBML spec.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `EBML`                | `0x1A45DFA3`| Master Element| 1           | The container for all EBML header elements.   |
| `EBMLVersion`         | `0x4286`   | Unsigned Int   | 1           | The version of the EBML parser required (e.g., 1). |
| `DocType`             | `0x4282`   | String         | 1           | The DocType of this file: `"GameCase"`.          |
| `DocTypeVersion`      | `0x4287`   | Unsigned Int   | 1           | The version of the DocType that was written (e.g., 1). |
| `DocTypeReadVersion`  | `0x4285`   | Unsigned Int   | 1           | The minimum DocType version a parser must support. |

#### 3.2. Root Element: `GameCase`

This element acts as the main container for the rest of the file's data.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `GameCase`            | `0x5B00`   | Master Element | 1           | The top-level element containing all file data.|

##### **Children of `GameCase`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Index`               | `0x5B01`   | Master Element | 0..1        | Contains pointers for fast seeking. **Must be the first child.**|
| `Game`                | `0x5B02`   | Master Element | 1           | Contains metadata about the file content.     |
| `Manuals`             | `0x5B03`   | Master Element | 0..1        | Contains game manuals in various formats.     |
| `RomHacks`            | `0x5B04`   | Master Element | 0..1        | Contains rom hack patches and related metadata.|
| `Media`               | `0x5B05`   | Master Element | 0..1        | Contains contains various media of the subject game of this file.|
| `GameCaseVersion`     | `0x5B06`   | Unsigned Int   | 1           | Contains the version of the GameCase format.  |


#### 3.3 `Index` Element
The `Index` element is used to provide pointers for fast seeking within the file. It should be the first child of the `GameCase` element. It contains one or more `IndexEntry` elements, each pointing to a top-level element within the file.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `IndexEntry`          | `0x5B01`   | Master Element | 1..n        | Contains a single index entry pointing to a top-level element.|

##### **Children of `IndexEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `IndexTargetID`       | `0x5B07`   | Binary         | 1           | The Element ID of the element being pointed to (e.g., the ID for Title). Stored as binary to preserve the VINT encoding.|
| `IndexPosition`       | `0x5B08`   | Unsigned Int   | 1           | The byte offset of the target element from the start of the GameCase element.|
| `IndexTarget`         | `0x5B09`   | UTF-8 String   | 1           | The type of content, e.g., "GameData", "Manual", "FanArt", "Video", "BoxArt". |
| `IndexTargetCategory` | `0x5B0A`   | UTF-8 String   | 0..1        | A specific category, e.g., "Front" or "Back" for BoxArt; "Gameplay" for Video. |
| `IndexTargetRegion`   | `0x5B0B`   | UTF-8 String   | 0..1        | The region of the target content, e.g "USA", "EUR". Primarily for manuals or regional art.|
| `IndexTargetUID`      | `0x5B0C`   | Unsigned Int   | 1           | A unique identifier for the target content, typically used for distinguishing between multiple instances of the same type of content.|


#### 3.4 `Game` Element

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Title`               | `0x2E8A00` | UTF-8 String   | 1           | The primary title of the game.                |
| `Developer`           | `0x2E8A01` | UTF-8 String   | 1..n        | The developer of the game content.            |
| `Genre`               | `0x2E8A02` | UTF-8 String   | 0..1        | The genre of the game content.                |
| `MinPlayers`          | `0x2E8A03` | Unsigned Int   | 0..1        | The minimum number of players required to play the game.|
| `MaxPlayers`          | `0x2E8A04` | Unsigned Int   | 0..1        | The maximum number of players allowed to play the game.|
| `GameSystem`          | `0x2E8A05` | UTF-8 String   | 1           | The game system for the subject game and it's data.|
| `Description`         | `0x2E8A1C` | UTF-8 String   | 0..1        | A detailed description of the game.           |
| `Rating`              | `0x2E8A06` | Master Element | 0..1        | The user or critical rating of the game.      |
| `RegionInfo`          | `0x2E8A07` | Master Element | 0..n        | Container for the region specific information about the game.|
| `GameData`            | `0x2E8A08` | Master Element | 0..1        | Container for storing game binary data.       |


##### 3.4.1 **Children of `Rating`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `RatingValue`         | `0x1E8A9B00`| UTF-8 String  | 1           | The rating value (e.g., "8.5", "4/5", "95%"). |
| `RatingSource`        | `0x1E8A9B01`| UTF-8 String  | 1           | The entity providing the rating (e.g., "IGN", "User").|

##### 3.4.2 **Children of `RegionInfo`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Region`              | `0x1E8A9B02`| UTF-8 String  | 1           | The region of this `Region` element.          |
| `Language`            | `0x1E8A9B03`| UTF-8 String  | 1..n        | The language of this `Region` element. Must follow ISO 639-1 codes for the representation of names of languages.|
| `ReleaseDate`         | `0x1E8A9B04`| Date          | 1           | The release date of this `Region` element.    |
| `Publisher`           | `0x1E8A9B05`| UTF-8 String  | 1           | The publisher of the game content of this `Region` element.|
| `AgeRating`           | `0x1E8A9B06`| UTF-8 String  | 1           | The age rating of the game content of this `Region` element.|


##### 3.4.3 `GameData` Element
This element is the container for all game binary data. It can hold multiple `GameDataEntry` elements, which allows for storing different versions or types of game data (e.g., a raw ROM and a compressed CHD version) within the same `.gcase` file.

| Element Name          | Element ID          | Type           | Cardinality | Description                          |
| :-------------------- | :------------------ | :------------- | :---------- | :----------------------------------- |
| `GameData`            | `0x2E8A08`          | Master Element | 0..1        | Container for one or more game data entries. |

###### **Children of `GameData`:**
| Element Name          | Element ID          | Type           | Cardinality | Description                          |
| :-------------------- | :------------------ | :------------- | :---------- | :----------------------------------- |
| `GameDataEntry`       | `0x2E8A09`          | Master Element | 1..n        | A container for a single instance of game data (e.g., a single ROM, a CHD set, or an archive). Multiple entries can be used to store different regional versions (e.g., USA and Japan) or different formats (e.g., a .iso and a .chd) of the same game within one .gcase file. |


#### 3.4.3.1 `GameDataEntry` Element
This is a self-contained entry for a game's binary data. The `DataFormat` element within it **must** be parsed first, as it defines which other elements are present within this entry.

| Element Name          | Element ID          | Type           | Cardinality | Description                          |
| :-------------------- | :------------------ | :------------- | :---------- | :----------------------------------- |
| `DataFormat`          | `0x2E8A0A`          | UTF-8 String   | 1           | Declares the format of the data. Must be one of: "RAW", "ARCHIVE", "CHD", "BIN/CUE", "SSMC". Case sensitive. |
| `RawData`             | `0x2E8A0B`          | Master Element | 0..1        | Contains data for a single raw game file. **Present if `DataFormat` is "RAW".** |
| `ArchiveData`         | `0x2E8A0C`          | Master Element | 0..1        | Contains data for a compressed archive. **Present if `DataFormat` is "ARCHIVE".** |
| `ChdData`             | `0x2E8A0D`          | Master Element | 0..1        | Contains data for a CHD collection. **Present if `DataFormat` is "CHD".** |
| `BinCueData`          | `0x2E8A0E`          | Master Element | 0..1        | Contains data for a BIN/CUE collection. **Present if `DataFormat` is "BIN/CUE".** |
| `SSMCData`            | `0x2E8A0F`          | Master Element | 0..1        | Contains data for a SpriteShrink MultiCart (.ssmc). **Present if `DataFormat` is "SSMC".** |


##### 3.4.3.1.1 `RawData` Element (Format: "RAW") 
Stores a single, uncompressed game file.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `FileProperties`      | `0x2E8A10` | Master Element | 1           | Contains properties of the file.              |
| `ROMData`             | `0x1E8AA000`| Binary        | 1           | The raw binary data of the game ROM.          |

**Children of `FileProperties`:**  
Stores the information for a single ROM. Used by each of the GameDataEntry children Master Elements.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ROMFileName`         | `0x1E8A9B07`| UTF-8 String  | 0..1        | The name of the file in the archive (omitted when format is RAW).|
| `ROMFileSize`         | `0x1E8A9B08`| Unsigned Int  | 1           | The size of the file.                         |
| `ROMRegion`           | `0x1E8A9B09`| UTF-8 String  | 1           | The region of this game file.                 |
| `SupportedLanguage`   | `0x1E8A9B0A`| UTF-8 String  | 1..n        | A supported language by the ROM.              |
| `CRC32`               | `0x1E8A9B0B`| Binary        | 0..1        | The 4-byte CRC32 hash of the file (uncompressed if in archive).|
| `MD5`                 | `0x1E8A9B0C`| Binary        | 0..1        | The 16-byte MD5 hash of the file (uncompressed if in archive).|
| `SHA1`                | `0x1E8A9B0D`| Binary        | 0..1        | The 20-byte SHA-1 hash of the file (uncompressed if in archive).|
| `SHA256`              | `0x1E8A9B0E`| Binary        | 0..1        | The SHA-256 hash of the file (uncompressed if in archive).|
| `SHA512`              | `0x1E8A9B0F`| Binary        | 0..1        | The SHA-512 hash of the file (uncompressed if in archive).|
| `SSMCIndex`           | `0x1E8A9B10`| Unsigned Int  | 0..1        | The SSMC index of the file.                   |


##### 3.4.3.1.2 `ArchiveData` Element (Format: "ARCHIVE")
Stores a single compressed file (zip, 7z, etc.) that may contain one or multiple ROMs. This allows inspecting archive contents without full decompression.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `CompressionAlgorithm`| `0x1E8A9B11`| UTF-8 String | 1           | The compression algorithm (e.g., "zip", "7z"). |
| `ArchivedFileCount`   | `0x1E8A9B12`| Unsigned Int  | 1           | The number of files described in the metadata. |
| `ArchiveFileMetadata` | `0x2E8A11` | Master Element | 1..n        | Contains metadata for each archived file.
| `ArchiveBinary`       | `0x1E8AA001`| Binary        | 1           | The complete binary data of the archive file. |

**Children of `ArchivedFileMetadata`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `FileProperties`      | `0x2E8A10` | Master Element | 1           | Contains properties of the file.              |


##### 3.4.3.1.3 `ChdData` Element (Format: "CHD")
Stores a collection of CHD files, typically for multi-disc games.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `CollectionRegion`    | `0x1E8A9B13`| UTF-8 String  | 1           | The primary region of this collection.        |
| `SupportedLanguage`   | `0x1E8A9B0A`| UTF-8 String  | 1..n        | A language supported by this set.             |
| `CHDCount`            | `0x1E8A9B14`| Unsigned Int  | 1           | The number of CHD files in this collection.   |
| `CHDEntry`            | `0x2E8A12` | Master Element | 1..n        | A container for a single CHD file.            |

**Children of `CHDEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `DiscNum`             | `0x1E8A9B15`| Unsigned Int  | 1           | The disc number for this CHD in the collection. |
| `FileProperties`      | `0x2E8A10`| Master Element  | 1           | Contains properties of the uncompressed file. |
| `CHDBinary`           | `0x1E8AA002`| Binary        | 1           | The binary data of the complete .chd file.    |


##### 3.4.3.1.4 `BinCueData` Element (Format: "BIN/CUE")
Stores a collection of BIN/CUE file pairs, typically for multi-disc games.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `CollectionRegion`    | `0x1E8A9B13`| UTF-8 String  | 1           | The primary region of this collection.        |
| `SupportedLanguage`   | `0x1E8A9B0A`| UTF-8 String  | 1..n        | A language supported by this set.             |
| `BinCueCount`         | `0x1E8A9B16`| Unsigned Int  | 1           | The number of BIN/CUE pairs in this collection. |
| `BinCueEntry`         | `0x2E8A13` | Master Element | 1..n        | A container for a single BIN/CUE pair.        |

**Children of `BinCueEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `DiscNum`             | `0x1E8A9B15`| Unsigned Int  | 1           | The disc number for this BIN file.            |
| `CueSheet`            | `0x1E8A9B17`| UTF-8 String  | 1           | The full text content of the .cue sheet.      |
| `FileProperties`      | `0x2E8A10` | Master Element | 1           | Contains properties of a bin/cue pair.        |
| `BinBinary`           | `0x1E8AA003`| Binary        | 1           | The binary data of the complete .bin file.    |


##### 3.4.3.1.5 `SSMCData` Element (Format: "SSMC")
Stores the metadata of an SpriteShrink MultiCart archive, typically contains all regsions and revisions of a single game.

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ArchivedFileCount`   | `0x1E8A9B12`| Unsigned Int  | 1           | The number of files described in the metadata.|
| `SSMCEntry`           | `0x2E8A14` | Master Element | 1..n        | Metadata for a file in the .ssmc file.        |
| `SSMCBinary`          | `0x1E8AA004`| Binary        | 1           | The complete binary data of the .ssmc file.   |

**Children of `SSMCEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `FileProperties`      | `0x2E8A10` | Master Element | 1           | Contains properties of a file in the archive. |


#### 3.5 `Manuals` Element

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ManualEntry`         | `0x2E8A15` | Master Element | 1..n        | Contains data for a single manual.            |

#### 3.5.1 `ManualEntry` Element
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Region`              | `0x1E8A9B02`| UTF-8 String  | 1           | Region where the manual was printed.          |
| `Language`            | `0x1E8A9B03`| UTF-8 String  | 1..n        | A language supported by the manual.           |
| `PageCount`           | `0x1E8A9B1A`| Unsigned Int  | 1           | The amount of pages the manual contains.      |
| `Revision`            | `0x1E8A9B1B`| UTF-8 String  | 1           | The revision number(s)/letter(s) of the manual.|
| `ManualFormat`        | `0x1E8A9B1C`| UTF-8 String  | 1           | The format of the manual (e.g., PDF, CBZ).    |
| `ManualData`          | `0x1E8AA005`| Binary Data   | 1           | The raw binary data of the manual file.       |


#### 3.6 `RomHacks` Element

| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `RomHackEntry`        | `0x2E8A16` | Master Element | 0..n        | Contains a single rom hack and its metadata.  |

#### 3.6.1 `ROMHackEntry` Element
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Title`               | `0x2E8A00` | UTF-8 String  | 1           | The title of the rom hack.                    |
| `Developer`           | `0x2E8A01` | UTF-8 String  | 0..n        | The author of the rom hack.                   |
| `HackVersion`         | `0x1E8A9B1F`| UTF-8 String  | 0..1        | The version of the rom hack.                  |
| `PatchFormat`         | `0x1E8A9B20`| UTF-8 String  | 0..1        | The format of the patch file (e.g., "IPS", "BPS", "UPS").|
| `TargetHash`          | `0x2E8A17`  | Master Element| 0..n        | The hash of the target game ROM.              |
| `HackDescription`     | `0x1E8A9B21`| UTF-8 String  | 0..1        | A description of the rom hack.                |
| `PatchData`           | `0x1E8AA006`| Binary        | 0..n        | The binary data of the patch file.            |

**Children of `TargetHash`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `HashFormat`          | `0x1E8A9B22`| UTF-8 String  | 0..n        | The format of the hash (e.g., "CRC32", "MD5", "SHA1").|
| `HashValue`           | `0x1E8A9B23`| Binary        | 1           | The binary hash data.                         |


#### 3.7 `Media` Element
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ImageCollection`     | `0x2E8A18` | Master Element | 0..n        | Contains a collection of related images.      |
| `VideoCollection`     | `0x2E8A1A` | Master Element | 0..n        | Contains a collection of related videos.      |

**Children of `ImageCollection`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Category`            | `0x1E8A9B24`| UTF-8 String  | 1           | Category of images (Example values: "BoxArt (Front)", "BoxArt (Back)", "Cartridge", "Disc Art", "FanArt", "Clear Logo", "Screenshot").|
| `ImageEntry`          | `0x2E8A19`| Master Element  | 1..n        | Container for a single image.                 |

**Children of `ImageEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `ImageTitle`          | `0x1E8A9B25`| UTF-8 String  | 0..1        | A title for the image.                        |
| `Artist`              | `0x1E8A9B26`| UTF-8 String  | 0..n        | The creator(s) of the artwork.                |
| `Region`              | `0x1E8A9B27`| UTF-8 String  | 0..1        | The specific region for this image, if applicable (e.g., "USA" for box art).|
| `ImageFormat`         | `0x1E8A9B28`| UTF-8 String  | 1           | The format of the image data (e.g., "PNG", "JPEG", "WEBP").|
| `Width`               | `0x1E8A9B29`| Unsigned Int  | 1           | The width of the image in pixels.             |
| `Height`              | `0x1E8A9B2A`| Unsigned Int  | 1           | The height of the image in pixels.            |
| `ImageData`           | `0x1E8AA007`| Binary        | 1           | The raw binary data of the image.             |

**Children of `VideoCollection`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `Category`            | `0x1E8A9B24`| UTF-8         | 1           | Category of videos (e.g. trailer, gameplay video, etc.)|
| `VideoEntry`          | `0x2E8A1B` | Master Element | 1..n        | Container for a single video.                 |

**Children of `VideoEntry`:**
| Element Name          | Element ID | Type           | Cardinality | Description                                   |
| :-------------------- | :--------- | :------------- | :---------- | :-------------------------------------------- |
| `VideoTitle`          | `0x1E8A9B2B`| UTF-8 String  | 0..1        | A title for the video.                        |
| `Language`            | `0x1E8A9B2C`| UTF-8 String  | 0..n        | The language of the video.                    |
| `Duration`            | `0x1E8A9B2D`| Unsigned Int  | 1           | The duration of the video in milliseconds.    |
| `VideoFormat`         | `0x1E8A9B2E`| UTF-8 String  | 1           | The container format of the video (e.g., MP4, AVI, MKV).|
| `Width`               | `0x1E8A9B29`| Unsigned Int  | 1           | The width of the video in pixels.             |
| `Height`              | `0x1E8A9B2A`| Unsigned Int  | 1           | The height of the video in pixels.            |
| `Thumbnail`           | `0x1E8AA008`| Binary        | 0..1        | A thumbnail image of the video.               |
| `VideoData`           | `0x1E8AA009`| Binary        | 1           | The raw binary data of the video file.        |

---


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
| 1.0     | 2025-07-16 | Tyler Bibeault | Initial version of the specification.       |

