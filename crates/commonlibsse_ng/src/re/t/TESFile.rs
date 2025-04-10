use crate::re::BSFile::BSFile;
use crate::re::BSString::BSString;
use crate::re::BSTList::BSSimpleList;
use crate::re::FORM::FORM;
use crate::re::TESBitArrayFile::TESBitArrayFile;
use crate::re::TESObjectCELL::TESObjectCELL;
use core::ffi::c_void;
use windows::Win32::{Foundation::FILETIME, Storage::FileSystem::WIN32_FIND_DATAA};

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    None = 0,
    NotFound = 1,
    NoFile = 2,
    NoForm = 3,
    NoChunk = 4,
    NoID = 5,
    BadFile = 6,
    BadID = 7,
    FormOpen = 8,
    FileOpen = 9,
    WriteFailure = 10,
    InvalidFile = 11,
    FileInUse = 12,
    CreateFailure = 13,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecordFlags: u32 {
        const None = 0;
        const Master = 1 << 0;
        const Altered = 1 << 1;
        const Checked = 1 << 2;
        const Active = 1 << 3;
        const OptimizedFile = 1 << 4;
        const TempIDOwner = 1 << 5;
        const Delocalized = 1 << 7;
        const PrecalcDataOnly = 1 << 8;
        const SmallFile = 1 << 9;
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TESFile {
    pub lastError: Error,                           // 000
    pub pad004: u32,                                // 004
    pub threadSafeParent: *mut TESFile,             // 008
    pub threadSafeFileMap: *mut c_void,             // 010
    pub unk018: u64,                                // 018
    pub unk020: u64,                                // 020
    pub unk028: u8,                                 // 028
    pub unk029: u8,                                 // 029
    pub pad02A: u16,                                // 02A
    pub pad02C: u32,                                // 02C
    pub lockedFile: *mut BSFile,                    // 030
    pub file: *mut BSFile,                          // 038
    pub formUserDataBitArray: *mut TESBitArrayFile, // 040
    pub formVersionBitArray: *mut TESBitArrayFile,  // 048
    pub formIDBitArray: *mut TESBitArrayFile,       // 050
    pub fileName: [u8; 260_usize],                  // 058
    pub path: [u8; 260_usize],                      // 15C
    pub buffer: *mut u8,                            // 260
    pub bufferAllocSize: u32,                       // 268
    pub firstCellOffset: u32,                       // 26C
    pub currCellOffset: u32,                        // 270
    pub unk274: u32,                                // 274
    pub currCell: *mut TESObjectCELL,               // 278
    pub currRefOffset: u32,                         // 280
    pub currentform: FORM,                          // 284
    pub currentchunkID: u32,                        // 29C
    pub actualChunkSize: u32,                       // 2A0
    pub filesize: u32,                              // 2A4
    pub fileOffset: u32,                            // 2A8
    pub formoffset: u32,                            // 2AC
    pub chunkoffset: u32,                           // 2B0
    pub saveform: FORM,                             // 2B4
    pub saveFormOffset: u32,                        // 2CC
    pub saveChunkOffset: u64,                       // 2D0
    pub unk2D8: u64,                                // 2D8
    pub unk2E0: u64,                                // 2E0
    pub unk2E8: u8,                                 // 2E8
    pub isBigEndian: bool,                          // 2E9
    pub unk2EA: u8,                                 // 2EA
    pub pad2EB: u8,                                 // 2EB
    pub fileData: WIN32_FIND_DATAA,                 // 2EC
    pub unk42C: f32,                                // 42C
    pub unk430: u32,                                // 430
    pub flags: u32,                                 // 434
    pub recordFlags: RecordFlags,                   // 438
    pub pad43C: u32,                                // 43C
    pub masters: BSSimpleList<*const u8>,           // 440
    pub mastersData: BSSimpleList<*mut u64>,        // 450
    pub masterCount: u32,                           // 460
    pub pad464: u32,                                // 464
    pub masterPtrs: *mut *mut TESFile,              // 468
    pub deletedFormTime: FILETIME,                  // 470
    pub compileIndex: u8,                           // 478
    pub pad479: u8,                                 // 479
    pub smallFileCompileIndex: u16,                 // 47A
    pub pad47C: u32,                                // 47C
    pub createdBy: BSString,                        // 480
    pub summary: BSString,                          // 490
    pub decompressedFormBuffer: *mut u8,            // 4A0
    pub decompressedFormBufferSize: u32,            // 4A8
    pub pad4AC: u32,                                // 4AC
    pub reservedDecompressionBuffer: *mut c_void,   // 4B0
    pub reservedDecompressionBufferSize: u32,       // 4B8
    pub pad4BC: u32,                                // 4BC
    pub unk4C0: *mut c_void,                        // 4C0
}

const_assert_eq!(core::mem::size_of::<TESFile>(), 0x4C8);

const _: () = {
    assert!(core::mem::offset_of!(TESFile, lastError) == 0x000);
    assert!(core::mem::offset_of!(TESFile, pad004) == 0x004);
    assert!(core::mem::offset_of!(TESFile, threadSafeParent) == 0x008);
    assert!(core::mem::offset_of!(TESFile, threadSafeFileMap) == 0x010);
    assert!(core::mem::offset_of!(TESFile, unk018) == 0x018);
    assert!(core::mem::offset_of!(TESFile, unk020) == 0x020);
    assert!(core::mem::offset_of!(TESFile, unk028) == 0x028);
    assert!(core::mem::offset_of!(TESFile, unk029) == 0x029);
    assert!(core::mem::offset_of!(TESFile, pad02A) == 0x02A);
    assert!(core::mem::offset_of!(TESFile, pad02C) == 0x02C);
    assert!(core::mem::offset_of!(TESFile, lockedFile) == 0x030);
    assert!(core::mem::offset_of!(TESFile, file) == 0x038);
    assert!(core::mem::offset_of!(TESFile, formUserDataBitArray) == 0x040);
    assert!(core::mem::offset_of!(TESFile, formVersionBitArray) == 0x048);
    assert!(core::mem::offset_of!(TESFile, formIDBitArray) == 0x050);
    assert!(core::mem::offset_of!(TESFile, fileName) == 0x058);
    assert!(core::mem::offset_of!(TESFile, path) == 0x15C);
    assert!(core::mem::offset_of!(TESFile, buffer) == 0x260);
    assert!(core::mem::offset_of!(TESFile, bufferAllocSize) == 0x268);
    assert!(core::mem::offset_of!(TESFile, firstCellOffset) == 0x26C);
    assert!(core::mem::offset_of!(TESFile, currCellOffset) == 0x270);
    assert!(core::mem::offset_of!(TESFile, unk274) == 0x274);
    assert!(core::mem::offset_of!(TESFile, currCell) == 0x278);
    assert!(core::mem::offset_of!(TESFile, currRefOffset) == 0x280);
    assert!(core::mem::offset_of!(TESFile, currentform) == 0x284);
    assert!(core::mem::offset_of!(TESFile, currentchunkID) == 0x29C);
    assert!(core::mem::offset_of!(TESFile, actualChunkSize) == 0x2A0);
    assert!(core::mem::offset_of!(TESFile, filesize) == 0x2A4);
    assert!(core::mem::offset_of!(TESFile, fileOffset) == 0x2A8);
    assert!(core::mem::offset_of!(TESFile, formoffset) == 0x2AC);
    assert!(core::mem::offset_of!(TESFile, chunkoffset) == 0x2B0);
    assert!(core::mem::offset_of!(TESFile, saveform) == 0x2B4);
    assert!(core::mem::offset_of!(TESFile, saveFormOffset) == 0x2CC);
    assert!(core::mem::offset_of!(TESFile, saveChunkOffset) == 0x2D0);
    assert!(core::mem::offset_of!(TESFile, unk2D8) == 0x2D8);
    assert!(core::mem::offset_of!(TESFile, unk2E0) == 0x2E0);
    assert!(core::mem::offset_of!(TESFile, unk2E8) == 0x2E8);
    assert!(core::mem::offset_of!(TESFile, isBigEndian) == 0x2E9);
    assert!(core::mem::offset_of!(TESFile, unk2EA) == 0x2EA);
    assert!(core::mem::offset_of!(TESFile, pad2EB) == 0x2EB);
    assert!(core::mem::offset_of!(TESFile, fileData) == 0x2EC);
    assert!(core::mem::offset_of!(TESFile, unk42C) == 0x42C);
    assert!(core::mem::offset_of!(TESFile, unk430) == 0x430);
    assert!(core::mem::offset_of!(TESFile, flags) == 0x434);
    assert!(core::mem::offset_of!(TESFile, recordFlags) == 0x438);
    assert!(core::mem::offset_of!(TESFile, pad43C) == 0x43C);
    assert!(core::mem::offset_of!(TESFile, masters) == 0x440);
    assert!(core::mem::offset_of!(TESFile, mastersData) == 0x450);
    assert!(core::mem::offset_of!(TESFile, masterCount) == 0x460);
    assert!(core::mem::offset_of!(TESFile, pad464) == 0x464);
    assert!(core::mem::offset_of!(TESFile, masterPtrs) == 0x468);
    assert!(core::mem::offset_of!(TESFile, deletedFormTime) == 0x470);
    assert!(core::mem::offset_of!(TESFile, compileIndex) == 0x478);
    assert!(core::mem::offset_of!(TESFile, pad479) == 0x479);
    assert!(core::mem::offset_of!(TESFile, smallFileCompileIndex) == 0x47A);
    assert!(core::mem::offset_of!(TESFile, pad47C) == 0x47C);
    assert!(core::mem::offset_of!(TESFile, createdBy) == 0x480);
    assert!(core::mem::offset_of!(TESFile, summary) == 0x490);
    assert!(core::mem::offset_of!(TESFile, decompressedFormBuffer) == 0x4A0);
    assert!(core::mem::offset_of!(TESFile, decompressedFormBufferSize) == 0x4A8);
    assert!(core::mem::offset_of!(TESFile, pad4AC) == 0x4AC);
    assert!(core::mem::offset_of!(TESFile, reservedDecompressionBuffer) == 0x4B0);
    assert!(core::mem::offset_of!(TESFile, reservedDecompressionBufferSize) == 0x4B8);
    assert!(core::mem::offset_of!(TESFile, pad4BC) == 0x4BC);
    assert!(core::mem::offset_of!(TESFile, unk4C0) == 0x4C0);
};
