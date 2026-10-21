use super::*;

/// Encodes bytes in the standard base64 alphabet.
///
/// # Arguments
///
/// - `&[u8]` - The bytes to encode.
///
/// # Returns
///
/// - `String` - The base64 text of the bytes.
pub fn base64_encode(data: &[u8]) -> String {
    let mut result: String = String::new();
    for chunk in data.chunks(3) {
        let first: u32 = chunk[0] as u32;
        let second: u32 = chunk.get(1).copied().unwrap_or(0) as u32;
        let third: u32 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple: u32 = (first << 16) | (second << 8) | third;
        result.push(BASE64_ALPHABET[((triple >> 18) & 0x3f) as usize] as char);
        result.push(BASE64_ALPHABET[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            result.push(BASE64_ALPHABET[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            result.push(BASE64_PADDING as char);
        }
        if chunk.len() > 2 {
            result.push(BASE64_ALPHABET[(triple & 0x3f) as usize] as char);
        } else {
            result.push(BASE64_PADDING as char);
        }
    }
    result
}

/// Decodes a standard base64 text into bytes.
///
/// The two characters of the url safe alphabet are accepted as well, so a
/// key that is handed over in either flavour can be read.
///
/// # Arguments
///
/// - `&str` - The base64 text to decode.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The bytes of the text, or the failure that stopped it.
pub fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    let mut result: Vec<u8> = Vec::new();
    for character in text.chars() {
        if character.is_whitespace() {
            continue;
        }
        if character as u32 == u32::from(BASE64_PADDING) {
            break;
        }
        let value: u32 = match base64_value(character) {
            Some(value) => value,
            None => return Err(ERROR_INVALID_BASE64.to_string()),
        };
        buffer = ((buffer << 6) | value) & BASE64_BUFFER_MASK;
        bits += 6;
        if bits >= BITS_BYTE {
            bits -= BITS_BYTE;
            result.push(((buffer >> bits) & 0xff) as u8);
        }
    }
    Ok(result)
}

/// Encrypts a credential with the public key of the server.
///
/// The server decrypts the credentials with the PKCS#1 v1.5 padding, so the
/// block is padded the same way and encrypted with the modular
/// exponentiation of the public key.
///
/// # Arguments
///
/// - `&str` - The credential to encrypt.
/// - `&PublicKey` - The public key of the server.
///
/// # Returns
///
/// - `Result<String, String>` - The base64 encoded cipher text, or the failure that stopped it.
pub fn encrypt_field(plaintext: &str, key: &PublicKey) -> Result<String, String> {
    let block: Vec<u8> = pad_block(plaintext.as_bytes(), key.block_size())?;
    let value: BigUint = BigUint::from_bytes_be(&block);
    if value.compare(&key.modulus()) != Ordering::Less {
        return Err(ERROR_MESSAGE_TOO_LONG.to_string());
    }
    let cipher: BigUint = value.modpow(&key.exponent(), &key.modulus());
    Ok(base64_encode(&cipher.to_bytes_be(key.block_size())))
}

/// Pads a credential the way PKCS#1 v1.5 pads it.
///
/// The padding bytes never hold zero, so the server finds the separator of
/// the block without reading past the message.
///
/// # Arguments
///
/// - `&[u8]` - The credential to pad.
/// - `usize` - The number of bytes the block holds.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The padded block, or the failure that stopped it.
fn pad_block(message: &[u8], size: usize) -> Result<Vec<u8>, String> {
    if message.len() + PADDING_OVERHEAD > size {
        return Err(ERROR_MESSAGE_TOO_LONG.to_string());
    }
    let mut block: Vec<u8> = vec![0u8; size];
    block[0] = BLOCK_PREFIX;
    block[1] = BLOCK_TYPE;
    let filler: u8 = padding_filler();
    let separator: usize = size - message.len() - 1;
    for index in 2..separator {
        block[index] = filler;
    }
    block[separator] = BLOCK_SEPARATOR;
    block[separator + 1..].copy_from_slice(message);
    Ok(block)
}

/// Returns the byte the padding of a block is filled with.
///
/// The byte is derived from the clock, so two blocks of the same credential
/// do not look alike, and it never holds zero.
///
/// # Returns
///
/// - `u8` - The byte the padding is filled with.
fn padding_filler() -> u8 {
    let now: i64 = js_sys::Date::now() as i64;
    ((now & 0x7f) as u8) + 1
}

/// Returns the value of one base64 character.
///
/// # Arguments
///
/// - `char` - The character to read.
///
/// # Returns
///
/// - `Option<u32>` - The value of the character, absent when it is unknown.
fn base64_value(character: char) -> Option<u32> {
    if character == '-' {
        return Some(62);
    }
    if character == '_' {
        return Some(63);
    }
    BASE64_ALPHABET
        .iter()
        .position(|byte: &u8| *byte == character as u8)
        .map(|index: usize| index as u32)
}
