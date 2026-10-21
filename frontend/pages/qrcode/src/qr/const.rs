/// The number of error correction codewords of every block.
///
/// The first index is the error correction level, the second one the
/// version of the code; the entry of the version zero is never used.
pub(crate) const ECC_CODEWORDS_PER_BLOCK: [[u8; 41]; 4] = [
    [
        0, 7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28, 30, 30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ],
    [
        0, 10, 16, 26, 18, 24, 16, 18, 22, 22, 26, 30, 22, 22, 24, 24, 28, 28, 26, 26, 26, 26, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    ],
    [
        0, 13, 22, 18, 26, 18, 24, 18, 22, 20, 24, 28, 26, 24, 20, 30, 24, 28, 28, 26, 30, 28, 30, 30, 30, 30, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ],
    [
        0, 17, 28, 22, 16, 22, 28, 26, 26, 24, 28, 24, 28, 22, 24, 24, 30, 28, 28, 26, 28, 30, 24, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ],
];

/// The number of error correction blocks of a code.
///
/// The first index is the error correction level, the second one the version
/// of the code; the entry of the version zero is never used.
pub(crate) const NUM_ERROR_CORRECTION_BLOCKS: [[u8; 41]; 4] = [
    [
        0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 4, 4, 4, 4, 4, 6, 6, 6, 6, 7, 8, 8, 9, 9, 10, 12, 12, 12, 13, 14, 15, 16, 17, 18, 19, 19, 20, 21, 22, 24, 25,
    ],
    [
        0, 1, 1, 1, 2, 2, 4, 4, 4, 5, 5, 5, 8, 9, 9, 10, 10, 11, 13, 14, 16, 17, 17, 18, 20, 21, 23, 25, 26, 28, 29, 31, 33, 35, 37, 38, 40, 43, 45, 47, 49,
    ],
    [
        0, 1, 1, 2, 2, 4, 4, 6, 6, 8, 8, 8, 10, 12, 16, 12, 17, 16, 18, 21, 20, 23, 23, 25, 27, 29, 34, 34, 35, 38, 40, 43, 45, 48, 51, 53, 56, 59, 62, 65, 68,
    ],
    [
        0, 1, 1, 2, 4, 4, 4, 5, 5, 8, 9, 9, 10, 12, 12, 16, 17, 16, 18, 21, 20, 23, 23, 25, 27, 29, 34, 34, 35, 38, 40, 43, 45, 48, 51, 53, 56, 59, 62, 65, 68,
    ],
];

/// The largest version a QR code can have.
pub(crate) const MAX_VERSION: usize = 40;

/// The number of quiet modules around a QR code.
pub(crate) const QUIET_ZONE: usize = 4;

/// The penalty of a run of five modules of the same colour.
pub(crate) const PENALTY_N1: u32 = 3;

/// The penalty of a block of two by two modules of the same colour.
pub(crate) const PENALTY_N2: u32 = 3;

/// The penalty of a pattern that looks like a finder pattern.
pub(crate) const PENALTY_N3: u32 = 40;

/// The penalty of an unbalanced amount of dark modules.
pub(crate) const PENALTY_N4: u32 = 10;

/// The mode indicator of the byte mode.
pub(crate) const MODE_BYTE: u32 = 0b0100;

/// The number of bits of the mode indicator.
pub(crate) const MODE_BITS: usize = 4;

/// The number of bits of the character count of the versions one to nine.
pub(crate) const COUNT_BITS_SHORT: usize = 8;

/// The number of bits of the character count of the versions ten to forty.
pub(crate) const COUNT_BITS_LONG: usize = 16;

/// The first padding byte of the data of a code.
pub(crate) const PAD_FIRST: u8 = 0xEC;

/// The second padding byte of the data of a code.
pub(crate) const PAD_SECOND: u8 = 0x11;

/// The radius of the finder pattern, its separator included.
pub(crate) const FINDER_RADIUS: i32 = 4;
