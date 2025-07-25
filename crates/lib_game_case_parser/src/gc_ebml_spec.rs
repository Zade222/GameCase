use ebml_iterable::specs::{ebml_specification, TagDataType};

#[ebml_specification]
#[derive(Clone, Debug, PartialEq)]
pub enum GCEbmlSpec {
    /*Start of EBML Header. */
    #[id(0x1A45DFA3)]
    #[data_type(TagDataType::Master)]
    EBML,

    #[id(0x4286)]
    #[data_type(TagDataType::UnsignedInt)]
    EBMLVersion,

    #[id(0x4282)]
    #[data_type(TagDataType::Utf8)]
    DocType,

    #[id(0x4287)]
    #[data_type(TagDataType::UnsignedInt)]
    DocTypeVersion,

    #[id(0x4285)]
    #[data_type(TagDataType::UnsignedInt)]
    DocTypeReadVersion,
    
    /*Root Element */
    #[id(0x5B00)]
    #[data_type(TagDataType::Master)]
    GameCase,

    /*Children of GameCase */
    #[id(0x5B01)]
    #[data_type(TagDataType::Master)]
    Index,

    #[id(0x5B02)]
    #[data_type(TagDataType::Master)]
    Game,

    #[id(0x5B03)]
    #[data_type(TagDataType::Master)]
    Manuals,

    #[id(0x5B04)]
    #[data_type(TagDataType::Master)]
    RomHacks,

    #[id(0x5B05)]
    #[data_type(TagDataType::Master)]
    Media,

    /*Index */
    #[id(0x5B06)]
    #[data_type(TagDataType::Master)]
    IndexEntry,

    /*Children of IndexEntry */
    #[id(0x5B07)]
    #[data_type(TagDataType::UnsignedInt)]
    IndexTargetID,

    #[id(0x5B08)]
    #[data_type(TagDataType::UnsignedInt)]
    IndexPosition,

    #[id(0x5B09)]
    #[data_type(TagDataType::Utf8)]
    IndexTarget,

    #[id(0x5B0A)]
    #[data_type(TagDataType::Utf8)]
    IndexTargetCategory,

    #[id(0x5B0B)]
    #[data_type(TagDataType::Utf8)]
    IndexTargetRegion,

    #[id(0x5B0C)]
    #[data_type(TagDataType::UnsignedInt)]
    IndexTargetUID,

    /*Game Element */
    #[id(0x2E8A00)]
    #[data_type(TagDataType::Utf8)]
    Title,

    #[id(0x2E8A01)]
    #[data_type(TagDataType::Utf8)]
    Developer,

    #[id(0x2E8A02)]
    #[data_type(TagDataType::Utf8)]
    Genre,

    #[id(0x2E8A03)]
    #[data_type(TagDataType::UnsignedInt)]
    MinPlayers,

    #[id(0x2E8A04)]
    #[data_type(TagDataType::UnsignedInt)]
    MaxPlayers,

    #[id(0x2E8A05)]
    #[data_type(TagDataType::Utf8)]
    GameSystem,

    #[id(0x2E8A1C)]
    #[data_type(TagDataType::Utf8)]
    Description,

    #[id(0x2E8A06)]
    #[data_type(TagDataType::Master)]
    Rating,

    #[id(0x2E8A07)]
    #[data_type(TagDataType::Master)]
    RegionInfo,

    #[id(0x2E8A08)]
    #[data_type(TagDataType::Master)]
    GameData,

    /*Children of Rating */
    #[id(0x1E8A9B00)]
    #[data_type(TagDataType::Utf8)]
    RatingValue,

    #[id(0x1E8A9B01)]
    #[data_type(TagDataType::Utf8)]
    RatingSource,

    /*Children of RegionInfo */
    #[id(0x1E8A9B02)]
    #[data_type(TagDataType::Utf8)]
    Region,

    #[id(0x1E8A9B03)]
    #[data_type(TagDataType::Utf8)]
    Language,

    #[id(0x1E8A9B04)]
    #[data_type(TagDataType::Binary)]//Meant to be type Date
    ReleaseDate,

    #[id(0x1E8A9B05)]
    #[data_type(TagDataType::Utf8)]
    Publisher,

    #[id(0x1E8A9B06)]
    #[data_type(TagDataType::Utf8)]
    AgeRating,

    /*Children of GameData Element */
    #[id(0x2E8A09)]
    #[data_type(TagDataType::Master)]
    GameDataEntry,

    /*Children of GataDataEntry Element */
    #[id(0x5B0D)]
    #[data_type(TagDataType::UnsignedInt)]
    EntryUID,

    #[id(0x2E8A0A)]
    #[data_type(TagDataType::Utf8)]
    DataFormat,

    #[id(0x2E8A0B)]
    #[data_type(TagDataType::Master)]
    RawData,

    #[id(0x2E8A0C)]
    #[data_type(TagDataType::Master)]
    ArchiveData,

    #[id(0x2E8A0D)]
    #[data_type(TagDataType::Master)]
    ChdData,

    #[id(0x2E8A0E)]
    #[data_type(TagDataType::Master)]
    BinCueData,

    #[id(0x2E8A0F)]
    #[data_type(TagDataType::Master)]
    SSMCData,

    /*RawData Element */
    #[id(0x2E8A10)]
    #[data_type(TagDataType::Master)]
    FileProperties,
    
    #[id(0x1E8AA000)]
    #[data_type(TagDataType::Binary)]
    ROMData,

    /*Children of FileProperties */
    #[id(0x1E8A9B07)]
    #[data_type(TagDataType::Utf8)]
    ROMFileName,

    #[id(0x1E8A9B08)]
    #[data_type(TagDataType::UnsignedInt)]
    ROMFileSize,

    /* Region already declared and is reused here.
    #[id(0x1E8A9B02)]
    #[data_type(TagDataType::Utf8)]
    Region,
    */

    /*Language already declared and is reused here.
    #[id(0x1E8A9B03)]
    #[data_type(TagDataType::Utf8)]
    Language,
    */

    #[id(0x1E8A9B0B)]
    #[data_type(TagDataType::Binary)]
    CRC32,

    #[id(0x1E8A9B0C)]
    #[data_type(TagDataType::Binary)]
    MD5,

    #[id(0x1E8A9B0D)]
    #[data_type(TagDataType::Binary)]
    SHA1,

    #[id(0x1E8A9B0E)]
    #[data_type(TagDataType::Binary)]
    SHA256,

    #[id(0x1E8A9B0F)]
    #[data_type(TagDataType::Binary)]
    SHA512,

    #[id(0x1E8A9B10)]
    #[data_type(TagDataType::UnsignedInt)]
    SSMCIndex,

    /*Children of ArchiveData */
    #[id(0x1E8A9B11)]
    #[data_type(TagDataType::Utf8)]
    CompressionAlgorithm,

    #[id(0x1E8A9B12)]
    #[data_type(TagDataType::UnsignedInt)]
    ArchivedFileCount,

    #[id(0x2E8A11)]
    #[data_type(TagDataType::Master)]
    ArchiveFileMetadata,

    #[id(0x1E8AA001)]
    #[data_type(TagDataType::Binary)]
    ArchiveBinary,

    /*Children of ArchivedFileMetadata */
    /*FileProperties already declared and is reused here.
    #[id(0x2E8A10)]
    #[data_type(TagDataType::Master)]
    FileProperties, 
    */

    /*Children of ChdData */
    /* Region already declared and is reused here.
    #[id(0x1E8A9B02)]
    #[data_type(TagDataType::Utf8)]
    Region,
    */

    /*Language already declared and is reused here.
    #[id(0x1E8A9B03)]
    #[data_type(TagDataType::Utf8)]
    Language,
    */

    #[id(0x1E8A9B14)]
    #[data_type(TagDataType::UnsignedInt)]
    CHDCount,

    #[id(0x2E8A12)]
    #[data_type(TagDataType::Master)]
    CHDEntry,

    /*Children of CHDEntry */
    #[id(0x1E8A9B15)]
    #[data_type(TagDataType::UnsignedInt)]
    DiscNum,

    /*FileProperties already declared and is reused here.
    #[id(0x2E8A10)]
    #[data_type(TagDataType::Master)]
    FileProperties, 
    */

    #[id(0x1E8AA002)]
    #[data_type(TagDataType::Binary)]
    CHDBinary,

    /*Children of BinCueData */
    /* Region already declared and is reused here.
    #[id(0x1E8A9B02)]
    #[data_type(TagDataType::Utf8)]
    Region,
    */

    /*Language already declared and is reused here.
    #[id(0x1E8A9B03)]
    #[data_type(TagDataType::Utf8)]
    Language,
    */

    #[id(0x1E8A9B16)]
    #[data_type(TagDataType::UnsignedInt)]
    BinCueCount,

    #[id(0x2E8A13)]
    #[data_type(TagDataType::Master)]
    BinCueEntry,

    /*Children of BinCueEntry */
    /*DiscNum already declared and is reused here.
    #[id(0x1E8A9B15)]
    #[data_type(TagDataType::UnsignedInt)]
    DiscNum,
    */

    #[id(0x1E8A9B17)]
    #[data_type(TagDataType::Utf8)]
    CueSheet,

    /*FileProperties already declared and is reused here.
    #[id(0x2E8A10)]
    #[data_type(TagDataType::Master)]
    FileProperties, 
    */

    #[id(0x1E8AA003)]
    #[data_type(TagDataType::Binary)]
    BinBinary,

    /*Children of SSMCData */
    /*ArchivedFileCount already declared and is reused here.
    #[id(0x1E8A9B12)]
    #[data_type(TagDataType::UnsignedInt)]
    ArchivedFileCount,
    */

    #[id(0x2E8A14)]
    #[data_type(TagDataType::Master)]
    SSMCEntry,

    #[id(0x1E8AA004)]
    #[data_type(TagDataType::Binary)]
    SSMCBinary,

    /*Children of SSMCEntry */
    /*FileProperties already declared and is reused here.
    #[id(0x2E8A10)]
    #[data_type(TagDataType::Master)]
    FileProperties, 
    */

    /*Manuals Element */
    #[id(0x2E8A15)]
    #[data_type(TagDataType::Master)]
    ManualEntry,

    /*Children of ManualEntry */
    /*EntryUID already declared and is reused here.
    #[id(0x5B0D)]
    #[data_type(TagDataType::UnsignedInt)]
    EntryUID,
    */

    /* Region already declared and is reused here.
    #[id(0x1E8A9B02)]
    #[data_type(TagDataType::Utf8)]
    Region,
    */

    /*Language already declared and is reused here.
    #[id(0x1E8A9B03)]
    #[data_type(TagDataType::Utf8)]
    Language,
    */

    #[id(0x1E8A9B1A)]
    #[data_type(TagDataType::UnsignedInt)]
    PageCount,

    #[id(0x1E8A9B1B)]
    #[data_type(TagDataType::Utf8)]
    Revision,

    #[id(0x1E8A9B1C)]
    #[data_type(TagDataType::Utf8)]
    ManualFormat,

    #[id(0x1E8AA005)]
    #[data_type(TagDataType::Binary)]
    ManualData,

    /*RomHacks Element */
    #[id(0x2E8A16)]
    #[data_type(TagDataType::Master)]
    RomHackEntry,

    /*Children of RomHackEntry */
    
}
