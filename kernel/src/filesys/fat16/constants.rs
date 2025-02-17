pub const SECTOR_SIZE: usize = 512;
pub const FAT_ENTRY_SIZE: usize = 2; // 16 bits per entry
pub const ROOT_DIR_ENTRIES: usize = 512;
pub const MAX_FILENAME_LENGTH: usize = 8;
pub const MAX_EXTENSION_LENGTH: usize = 3;
pub const ATTR_READ_ONLY: u8 = 0x01;
pub const ATTR_DIRECTORY: u8 = 0x10;
pub const ATTR_ARCHIVE: u8 = 0x20;
pub const DELETED_ENTRY_MARKER: u8 = 0xE5;
