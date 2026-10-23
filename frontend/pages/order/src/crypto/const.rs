/// The alphabet of the standard base64 encoding.
pub(crate) const BASE64_ALPHABET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The padding character of the standard base64 encoding.
pub(crate) const BASE64_PADDING: u8 = b'=';

/// The mask that keeps the buffer of the base64 decoder small.
pub(crate) const BASE64_BUFFER_MASK: u32 = 0xffff;

/// The number of bits one limb of the big integer holds.
pub(crate) const LIMB_BITS: u32 = 32;

/// The number of bits one byte holds.
pub(crate) const BITS_BYTE: u32 = 8;

/// The number of bytes one limb of the big integer holds.
pub(crate) const LIMB_BYTES: usize = 4;

/// The first byte of a PKCS#1 v1.5 encrypted block.
pub(crate) const BLOCK_PREFIX: u8 = 0x00;

/// The block type of a PKCS#1 v1.5 encrypted block.
pub(crate) const BLOCK_TYPE: u8 = 0x02;

/// The separator between the padding and the message of a PKCS#1 v1.5 block.
pub(crate) const BLOCK_SEPARATOR: u8 = 0x00;

/// The number of bytes the padding of a PKCS#1 v1.5 block costs.
pub(crate) const PADDING_OVERHEAD: usize = 11;

/// The failure reported when the public key cannot be decoded.
pub(crate) const ERROR_INVALID_KEY: &str = "the public key of the server is not valid";

/// The failure reported when a credential is longer than the key allows.
pub(crate) const ERROR_MESSAGE_TOO_LONG: &str = "the credential is longer than the key allows";

/// The failure reported when a base64 text holds an unknown character.
pub(crate) const ERROR_INVALID_BASE64: &str = "the base64 text holds an unknown character";
